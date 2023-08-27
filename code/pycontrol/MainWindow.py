import datetime
from functools import partial
from PySide6 import QtCore
from PySide6 import Qt
from PySide6.QtGui import QIntValidator, QKeyEvent
from PySide6.QtWidgets import QApplication, QWidget, QVBoxLayout, QHBoxLayout, QGridLayout, QPushButton, QSizePolicy, \
    QLabel, QSpinBox, QCheckBox, QDial, QSlider, QLCDNumber, QProgressBar, QLineEdit, QPlainTextEdit
from PySide6.QtCore import Qt, QTimer, Slot, Signal, QObject, QEvent
from threading import Thread, Lock
import evdev
import matplotlib.pyplot as plt

from dronecontrol import COMMAND_INPUT_MAX, THRUST_MAX, Drone, DroneVariable, THRUST_VARIABLE_NAME, ROLL_VARIABLE_NAME, \
    PITCH_VARIABLE_NAME, YAW_VARIABLE_NAME

EXPAND_EVERYWHERE_POLICY = QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
EXPAND_MIN_HORIZONTAL = QSizePolicy(QSizePolicy.Policy.Minimum, QSizePolicy.Policy.Expanding)
LOG_LENGTH = 100


class MainWindow(QWidget):
    line_edits: dict[str, QLineEdit]
    pressed_trims: dict[str, int]
    pressed_commands: dict[str, int]
    command_inputs: dict[int, (str, int)]
    trim_inputs: dict[int, (str, int)]
    setter_inputs: dict[int, (str, int)]

    status_box: QPlainTextEdit
    rewrite_status_checkbox: QCheckBox

    joystick: evdev.InputDevice
    joystick_thread: Thread
    joystick_lock: Lock

    logged_data: list[list[tuple]]

    def __init__(self):
        super().__init__()
        self.drone = Drone()
        self.setup_timer()
        self.pressed_trims = {}
        self.line_edits = {}
        self.command_inputs = {}
        self.pressed_commands = {}
        self.setter_inputs = {}
        self.logged_data = []
        self.ax = None

        self.yaw_variable = self.drone.variables[YAW_VARIABLE_NAME]
        self.pitch_variable = self.drone.variables[PITCH_VARIABLE_NAME]
        self.roll_variable = self.drone.variables[ROLL_VARIABLE_NAME]
        self.thrust_variable = self.drone.variables[THRUST_VARIABLE_NAME]

        self.setup_layout()
        self.setup_keymap()

        for device in evdev.list_devices():
            device = evdev.InputDevice(device)
            if "Joystick" in device.name:
                self.joystick = device
                self.joystick_lock = Lock()
                self.joystick_thread = Thread(target=self.joystick_listener, daemon=True)
                self.joystick_thread.start()

    def keyPressEvent(self, event: QKeyEvent):
        if not event.isAutoRepeat():
            self.handle_command_press(event.key())
            self.handle_trim_press(event.key())
            if event.key() == Qt.Key.Key_X:
                self.drone_stop()
        super().keyPressEvent(event)

    def keyReleaseEvent(self, event: QKeyEvent):
        if not event.isAutoRepeat():
            self.handle_command_release(event.key())
            self.handle_trim_release(event.key())
            self.handle_setter_key(event.key())

        super().keyPressEvent(event)

    def joystick_listener(self):
        for event in self.joystick.read_loop():
            if event.type == evdev.ecodes.EV_KEY:
                print(evdev.categorize(event))
                if event.value == 1:
                    self.handle_command_press(("J", event.code))
                    self.handle_trim_press(("J", event.code))
                elif event.value == 0:
                    self.handle_command_release(("J", event.code))
                    self.handle_trim_release(("J", event.code))
                    self.handle_setter_key(("J", event.code))
                if event.code == evdev.ecodes.BTN_BASE3:
                    self.drone_stop()

    def setup_keymap(self):
        self.command_inputs = {
            Qt.Key.Key_A: (ROLL_VARIABLE_NAME, -1),
            Qt.Key.Key_D: (ROLL_VARIABLE_NAME, 1),
            Qt.Key.Key_W: (PITCH_VARIABLE_NAME, -1),
            Qt.Key.Key_S: (PITCH_VARIABLE_NAME, 1),
        }

        self.trim_inputs = {
            Qt.Key.Key_P: (THRUST_VARIABLE_NAME, 1),
            Qt.Key.Key_L: (THRUST_VARIABLE_NAME, -1),
            Qt.Key.Key_Q: (YAW_VARIABLE_NAME, -1),
            Qt.Key.Key_E: (YAW_VARIABLE_NAME, 1),
            ("J", evdev.ecodes.BTN_PINKIE): (YAW_VARIABLE_NAME, 1),
            ("J", evdev.ecodes.BTN_TOP2): (YAW_VARIABLE_NAME, -1),
            ("J", evdev.ecodes.BTN_BASE): (THRUST_VARIABLE_NAME, -1),
            ("J", evdev.ecodes.BTN_BASE2): (THRUST_VARIABLE_NAME, 1)
        }

        self.setter_inputs = {
            Qt.Key.Key_C: (THRUST_VARIABLE_NAME, 1010),
            ("J", evdev.ecodes.BTN_BASE4): (THRUST_VARIABLE_NAME, 1010)
        }

    def handle_command_press(self, key: int | tuple[str, int]):
        if key in self.command_inputs:
            variable_name, sign = self.command_inputs[key]
            self.drone.variables[variable_name].set_input(sign)
            self.draw_status()

    def handle_command_release(self, key: int | tuple[str, int]):
        if key in self.command_inputs:
            variable_name, sign = self.command_inputs[key]
            self.drone.variables[variable_name].set_input(0)
            self.draw_status()

    def handle_trim_press(self, key: int | tuple[str, int]):
        if key in self.trim_inputs:
            variable_name, sign = self.trim_inputs[key]
            self.pressed_trims[variable_name] = sign

    def handle_trim_release(self, key: int | tuple[str, int]):
        if key in self.trim_inputs:
            variable_name, sign = self.trim_inputs[key]
            if variable_name in self.pressed_trims:
                del self.pressed_trims[variable_name]

    def handle_setter_key(self, key: int | tuple[str, int]):
        if key in self.setter_inputs:
            variable_name, new_value = self.setter_inputs[key]
            self.drone.variables[variable_name].trim_set(new_value)

    def setup_layout(self):
        self.setup_base_layout()
        self.setup_top_menu()
        self.setup_status_grid()
        self.setup_variable_list()
        self.setup_status_box()

    def setup_base_layout(self):
        self.base_layout = QGridLayout(self)
        self.layout = self.base_layout
        self.top_menu = QHBoxLayout()
        self.status_grid = QGridLayout()
        self.property_list = QGridLayout()

        self.fill(self.base_layout, 1, 0, EXPAND_EVERYWHERE_POLICY)
        self.fill(self.base_layout, 1, 1, EXPAND_MIN_HORIZONTAL)
        self.fill(self.base_layout, 1, 2, EXPAND_EVERYWHERE_POLICY)

        self.base_layout.addLayout(self.top_menu, 0, 0, 1, 2)
        self.base_layout.addLayout(self.status_grid, 1, 0)
        self.base_layout.addLayout(self.property_list, 1, 1)

    def setup_status_box(self):
        self.rewrite_status_checkbox = QCheckBox("Rewrite status")
        self.rewrite_status_checkbox.setChecked(True)

        self.status_box = QPlainTextEdit()
        self.status_box.setEnabled(False)

        self.base_layout.addWidget(self.rewrite_status_checkbox, 0, 2)
        self.base_layout.addWidget(self.status_box, 1, 2)

    def setup_top_menu(self):
        top_menu_buttons = [
            ("Connect", self.bt_connect),
            ("Start", None),
            ("Stop", self.drone_stop),
            ("Plot", self.plot)
        ]
        for text, callback in top_menu_buttons:
            button = QPushButton(text=text)
            if callback is not None:
                button.clicked.connect(callback)
            self.top_menu.addWidget(button)

    def setup_status_grid(self):
        status_grid_rows = 7
        status_grid_columns = 7
        add_buttons = True

        for row in range(status_grid_rows):
            for column in range(status_grid_columns):
                self.fill(self.status_grid, row, column, EXPAND_MIN_HORIZONTAL)

        self.status_grid.setHorizontalSpacing(20)
        self.status_grid.setVerticalSpacing(20)

        self.roll_bar = QProgressBar()
        self.roll_bar.setRange(-COMMAND_INPUT_MAX, COMMAND_INPUT_MAX)
        self.roll_bar.setOrientation(Qt.Orientation.Horizontal)
        self.roll_bar.setTextVisible(False)
        self.status_grid.addWidget(self.roll_bar, 0, 1, 1, 5)

        self.roll_lcd = QLCDNumber(4)
        self.status_grid.addWidget(self.roll_lcd, 1, 3)

        self.pitch_bar = QProgressBar()
        self.pitch_bar.setRange(-COMMAND_INPUT_MAX, COMMAND_INPUT_MAX)
        self.pitch_bar.setOrientation(Qt.Orientation.Vertical)
        self.pitch_bar.setTextVisible(False)
        self.status_grid.addWidget(self.pitch_bar, 1, 6, 5, 1)

        self.pitch_lcd = QLCDNumber(4)
        self.status_grid.addWidget(self.pitch_lcd, 3, 5)

        self.yaw_dial = QDial()
        self.yaw_dial.setRange(-COMMAND_INPUT_MAX, COMMAND_INPUT_MAX)
        self.yaw_dial.setValue(0)
        self.yaw_dial.setEnabled(False)
        self.status_grid.addWidget(self.yaw_dial, 3, 3)

        self.yaw_lcd = QLCDNumber(4)
        self.status_grid.addWidget(self.yaw_lcd, 4, 3)

        self.thrust_bar = QProgressBar()
        self.thrust_bar.setRange(0, THRUST_MAX)
        self.thrust_bar.setOrientation(Qt.Orientation.Vertical)
        self.thrust_bar.setTextVisible(False)
        self.status_grid.addWidget(self.thrust_bar, 0, 0, 6, 1)

        self.thrust_lcd = QLCDNumber(4)
        self.status_grid.addWidget(self.thrust_lcd, 6, 0)

    def setup_variable_list(self):
        variables_to_skip = [
            THRUST_VARIABLE_NAME
        ]
        row_count = 0
        for variable_name in self.drone.variables:
            if variable_name in variables_to_skip:
                continue

            variable = self.drone.variables[variable_name]
            label = QLabel(text=variable_name)
            editor = QLineEdit(str(variable.trim_value))
            editor.setValidator(QIntValidator(variable.min_value, variable.max_value, self))
            increment_button = QPushButton(text="+")
            decrement_button = QPushButton(text="-")

            increment_button.pressed.connect(partial(self.trim_press, variable_name, 1))
            decrement_button.pressed.connect(partial(self.trim_press, variable_name, -1))

            increment_button.released.connect(partial(self.trim_release, variable_name))
            decrement_button.released.connect(partial(self.trim_release, variable_name))

            editor.editingFinished.connect(partial(self.variable_edit, editor, variable))

            self.property_list.addWidget(label, row_count, 0)
            self.property_list.addWidget(editor, row_count, 1)
            self.property_list.addWidget(increment_button, row_count, 2)
            self.property_list.addWidget(decrement_button, row_count, 3)

            self.line_edits[variable_name] = editor

            row_count += 1

    def trim_press(self, variable_name: str, sign: int):
        self.pressed_trims[variable_name] = sign

    def trim_release(self, variable_name):
        if variable_name in self.pressed_trims:
            del self.pressed_trims[variable_name]

    def variable_edit(self, editor: QLineEdit, variable: DroneVariable):
        old_trim, new_trim = variable.trim_set(int(editor.text()))
        if old_trim != new_trim:
            editor.setText(str(new_trim))
        self.draw_status()

    def setup_timer(self):
        self.periodic_timer = QTimer()
        self.periodic_timer.setInterval(50)
        self.periodic_timer.timeout.connect(self.timer_tick)
        self.periodic_timer.start()

    def update_variables(self):
        for variable_name in self.pressed_trims:
            self.update_trim(variable_name, self.pressed_trims[variable_name])

    def update_trim(self, variable_name, sign):
        old_trim, new_trim = self.drone.variables[variable_name].trim_increment(sign)

        if (old_trim != new_trim) and (variable_name in self.line_edits):
            self.line_edits[variable_name].setText(str(new_trim))

    @Slot()
    def timer_tick(self):
        self.update_variables()
        self.draw_status()
        self.update_telemetry(self.drone.get_telemetry_snapshot())

    def update_telemetry(self, snapshot: dict):
        # if not self.rewrite_status_checkbox.isChecked() and self.thrust_variable.trim_value < 1000:
        #    return
        values_list = [(val_id, snapshot[val_id]) for val_id in snapshot]
        values_list.sort(key=lambda x: x[0])
        logged_list = [(-1, datetime.datetime.now())] + values_list
        self.logged_data.append(logged_list)
        print(logged_list)
        self.logged_data = self.logged_data[-LOG_LENGTH:]

        text_line = []
        for value in values_list:
            val_id, val_val = value
            if type(val_val) == int:
                text_line.append(f"{val_val:10d}")
            else:
                text_line.append(f"{val_val:10.2f}")
        text_line = ",".join(text_line)
        if not self.rewrite_status_checkbox.isChecked():
            text_line = self.status_box.toPlainText() + "\n" + text_line
        self.status_box.setPlainText(text_line)
        self.status_box.verticalScrollBar().setValue(self.status_box.verticalScrollBar().maximum())

    def get_filler(self, size_policy):
        dummy_label = QLabel()
        dummy_label.setSizePolicy(size_policy)
        return dummy_label

    def fill(self, grid: QGridLayout, row: int, column: int, size_policy):
        filler = self.get_filler(size_policy)
        grid.addWidget(filler, row, column)

    @QtCore.Slot()
    def bt_connect(self):
        self.drone.reconnect()

    @QtCore.Slot()
    def drone_stop(self):
        self.drone.stop()

    def draw_status(self):
        self.roll_bar.setValue(self.roll_variable.get_cumulative_value())
        self.roll_lcd.display(self.roll_variable.get_cumulative_value())

        self.yaw_dial.setValue(self.yaw_variable.get_cumulative_value())
        self.yaw_lcd.display(self.yaw_variable.get_cumulative_value())

        self.pitch_bar.setValue(self.pitch_variable.get_cumulative_value())
        self.pitch_lcd.display(self.pitch_variable.get_cumulative_value())

        self.thrust_bar.setValue(self.thrust_variable.get_cumulative_value())
        self.thrust_lcd.display(self.thrust_variable.get_cumulative_value())

    @QtCore.Slot()
    def plot(self):
        if self.ax is None:
            plt.ion()
            self.fig, self.ax = plt.subplots()
            self.line, = self.ax.plot([(x[0][1]-self.logged_data[0][0][1]).total_seconds() for x in self.logged_data], [x[1][1] for x in self.logged_data])
            plt.plot()

        self.line.set_ydata([x[1][1] for x in self.logged_data])
        self.line.set_xdata([(x[0][1]-self.logged_data[0][0][1]).total_seconds() for x in self.logged_data])

        self.fig.canvas.draw()
        self.fig.canvas.flush_events()


def start_app():
    app = QApplication([])

    main_window = MainWindow()
    main_window.resize(1000, 600)
    main_window.show()

    app.exec()
