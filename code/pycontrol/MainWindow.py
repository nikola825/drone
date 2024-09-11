import datetime
from functools import partial
from PySide6 import QtCore
from PySide6 import Qt
from PySide6.QtGui import QIntValidator, QKeyEvent
from PySide6.QtWidgets import QApplication, QWidget, QVBoxLayout, QHBoxLayout, QGridLayout, QPushButton, QSizePolicy, \
    QLabel, QSpinBox, QCheckBox, QDial, QSlider, QLCDNumber, QProgressBar, QLineEdit, QPlainTextEdit, QComboBox
from PySide6.QtCore import Qt, QTimer, Slot, Signal, QObject, QEvent
from threading import Thread, Lock
import evdev
from evdev import ecodes
import matplotlib.pyplot as plt

from dronecontrol import COMMAND_INPUT_MAX, THRUST_MAX, Drone, DroneVariable, THRUST_VARIABLE_NAME, ROLL_VARIABLE_NAME, \
    PITCH_VARIABLE_NAME, YAW_VARIABLE_NAME

EXPAND_EVERYWHERE_POLICY = QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
EXPAND_MIN_HORIZONTAL = QSizePolicy(QSizePolicy.Policy.Minimum, QSizePolicy.Policy.Expanding)
LOG_LENGTH = 100
PLOTTED_VALUE_ID = 1
plot_enabled = False

bandwidths = ["DLPF_BW_256",
            "DLPF_BW_188",
            "DLPF_BW_98",
            "DLPF_BW_42",
            "DLPF_BW_20",
            "DLPF_BW_10",
            "DLPF_BW_5"]

rates = ["0", "1", "2", "3", "4"]

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

    plotted_values: list[tuple[datetime.datetime, int | float]]

    plot_refresh: bool

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
        self.fig = None
        self.plotted_values = []
        self.line = None

        self.yaw_variable = self.drone.variables[YAW_VARIABLE_NAME]
        self.pitch_variable = self.drone.variables[PITCH_VARIABLE_NAME]
        self.roll_variable = self.drone.variables[ROLL_VARIABLE_NAME]
        self.thrust_variable = self.drone.variables[THRUST_VARIABLE_NAME]

        self.setup_layout()
        self.setup_keymap()

        for device in evdev.list_devices():
            device = evdev.InputDevice(device)
            print(device.name)
            if "EdgeTX RM TX16S Joystick":
                self.joystick = device
                self.joystick_lock = Lock()
                self.joystick_thread = Thread(target=self.joystick_listener, daemon=True)
                self.joystick_thread.start()

    def keyPressEvent(self, event: QKeyEvent):
        if not event.isAutoRepeat():
            self.key_press_release_handler(event.key(), True)
            if event.key() == Qt.Key.Key_X:
                self.drone_stop()
        super().keyPressEvent(event)

    def keyReleaseEvent(self, event: QKeyEvent):
        if not event.isAutoRepeat():
            self.key_press_release_handler(event.key(), False)

        super().keyPressEvent(event)

    def key_press_release_handler(self, key: int|str, press:bool):
        print(key)
        if press:
            self.handle_command_press(key)
            self.handle_trim_press(key)
        else:
            self.handle_command_release(key)
            self.handle_trim_release(key)
            self.handle_setter_key(key)

    def halt(self):
        self.drone_stop()
        self.start_motors()

    def joystick_listener(self):
        joy_to_key_map = {
            (ecodes.EV_ABS, ecodes.ABS_RZ, 2047): (["THRUST_LOW"], False),
        }
        joy_to_function_map = {
            (ecodes.EV_ABS, ecodes.ABS_RZ, 0): (self.drone_stop, []),
            (ecodes.EV_ABS, ecodes.ABS_RZ, 1024): ( self.halt, []),
            (ecodes.EV_ABS, ecodes.ABS_THROTTLE, 2047): ( self.bt_connect, []),
        }
        analogs = {
            ecodes.ABS_Z: (0, 2048, 0, 500, THRUST_VARIABLE_NAME),
            ecodes.ABS_Y: (0, 2048, -90, 90, ROLL_VARIABLE_NAME),
            ecodes.ABS_X: (0, 2048, 90, -90 , PITCH_VARIABLE_NAME),
            ecodes.ABS_RX: (0, 2048, 90, -90, YAW_VARIABLE_NAME),
        }

        for event in self.joystick.read_loop():
            if event.type == ecodes.EV_ABS:
                if event.code in analogs:
                    in_range_low, in_range_high, out_range_low, out_range_high, variable_name = analogs[event.code]
                    var_val = int((event.value-in_range_low)/(in_range_high-in_range_low)*(out_range_high-out_range_low)+out_range_low)
                    self.drone.variables[variable_name].set_analog(var_val)
            if (event.type, event.code, event.value) in joy_to_key_map:
                keys, press = joy_to_key_map[(event.type, event.code, event.value)]
                for k in keys:
                    self.key_press_release_handler(k, press)
            if (event.type, event.code, event.value) in joy_to_function_map:
                f, a = joy_to_function_map[(event.type, event.code, event.value)]
                f(*a)

            print(evdev.categorize(event), event.value)

    def setup_keymap(self):
        self.command_inputs = {
            Qt.Key.Key_A: (ROLL_VARIABLE_NAME, -1),
            Qt.Key.Key_D: (ROLL_VARIABLE_NAME, 1),
            Qt.Key.Key_W: (PITCH_VARIABLE_NAME, -1),
            Qt.Key.Key_S: (PITCH_VARIABLE_NAME, 1),
            Qt.Key.Key_Q: (YAW_VARIABLE_NAME, 1),
            Qt.Key.Key_E: (YAW_VARIABLE_NAME, -1),
            "JOY_TR": (YAW_VARIABLE_NAME, -1),
            "JOY_TL": (YAW_VARIABLE_NAME, 1),
        }

        self.trim_inputs = {
            Qt.Key.Key_P: (THRUST_VARIABLE_NAME, 1),
            Qt.Key.Key_L: (THRUST_VARIABLE_NAME, -1),
            "JOY_UP": (THRUST_VARIABLE_NAME, 1),
            "JOY_DOWN": (THRUST_VARIABLE_NAME, -1)

        }

        self.setter_inputs = {
            Qt.Key.Key_C: (THRUST_VARIABLE_NAME, 800),
            Qt.Key.Key_V: (THRUST_VARIABLE_NAME, 1090),
            "THRUST_LOW": (THRUST_VARIABLE_NAME, 900),
            "JOY_GREEN": (THRUST_VARIABLE_NAME, 1090),
        }

    def handle_command_press(self, key: int | str):
        if key in self.command_inputs:
            variable_name, sign = self.command_inputs[key]
            self.drone.variables[variable_name].set_input(sign)
            self.draw_status()

    def handle_command_release(self, key: int | str):
        if key in self.command_inputs:
            variable_name, sign = self.command_inputs[key]
            self.drone.variables[variable_name].set_input(0)
            self.draw_status()

    def handle_trim_press(self, *args: str|int):
        for key in args:
            if key in self.trim_inputs:
                variable_name, sign = self.trim_inputs[key]
                self.pressed_trims[variable_name] = sign

    def handle_trim_release(self, *args: str|int):
        for key in args:
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
            ("Start", self.start_motors),
            ("Stop", self.drone_stop),
            ("Dump", self.sensor_dump),
            ("Fplot", self.frequency_plot)
        ]
        for text, callback in top_menu_buttons:
            button = QPushButton(text=text)
            if callback is not None:
                button.clicked.connect(callback)
            self.top_menu.addWidget(button)

        bw_dropdown=QComboBox()
        bw_dropdown.addItems(bandwidths)
        bw_dropdown.setCurrentIndex(3)
        bw_dropdown.currentIndexChanged.connect(self.select_bandwidth)
        self.top_menu.addWidget(bw_dropdown)

        self.rate_dropdown=QComboBox()
        self.rate_dropdown.addItems(rates)
        self.rate_dropdown.setCurrentIndex(0)
        self.rate_dropdown.currentIndexChanged.connect(self.select_rate)
        self.rate_dropdown.setEditable(False)
        self.top_menu.addWidget(self.rate_dropdown)

    def select_bandwidth(self, bandwidth_index):
        self.drone.set_bandwidth(bandwidth_index)

    def select_rate(self, rate_index):
        self.drone.set_rate(rate_index)
        self.rate_dropdown.clearFocus()
        self.setFocus()

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
        self.periodic_timer.setInterval(25)
        self.periodic_timer.timeout.connect(self.timer_tick)
        self.periodic_timer.start()

    def update_variables(self):
        for variable_name in self.pressed_trims:
            self.update_trim(variable_name, self.pressed_trims[variable_name])

    def update_trim(self, variable_name, sign):
        old_trim, new_trim = self.drone.variables[variable_name].trim_increment(sign)

        if (old_trim != new_trim) and (variable_name in self.line_edits):
            self.line_edits[variable_name].setText(str(new_trim))

    def start_motors(self):
        self.drone.start()

    @Slot()
    def timer_tick(self):
        self.update_variables()
        self.draw_status()
        #self.update_telemetry(self.drone.get_telemetry_snapshot())

    def update_telemetry(self, snapshot: dict):
        # if not self.rewrite_status_checkbox.isChecked() and self.thrust_variable.trim_value < 1000:
        #    return
        values_list = [(val_id, snapshot[val_id]) for val_id in snapshot]
        values_list.sort(key=lambda x: x[0])

        text_line = []


        timestamp = datetime.datetime.now()

        for value in values_list:
            val_id, val_val = value
            if type(val_val) == int:
                text_line.append(f"{val_val:10d}")
            else:
                text_line.append(f"{val_val:10.2f}")
            if val_id == PLOTTED_VALUE_ID:
                self.plotted_values.append((timestamp, val_val))

        self.plotted_values = self.plotted_values[-LOG_LENGTH:]

        self.plot()

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

    def plot(self):
        if not plot_enabled:
            return
        timestamps = [x[0] for x in self.plotted_values]

        time_start = 0
        if timestamps:
            time_start = timestamps[0]
            timestamps = [(x-time_start).total_seconds() for x in timestamps]

        values = [x[1] for x in self.plotted_values]
        min_value = 0
        max_value = 0
        if values:
            min_value = min(values)
            max_value = max(values)

        if self.ax is None:
            plt.ion()
            self.fig, self.ax = plt.subplots()
            self.line, = self.ax.plot([], [])
            plt.plot()
        elif self.thrust_variable.trim_value >= 0:
            self.line.set_ydata(values)
            self.line.set_xdata(timestamps)
            self.ax.set_ylim(min_value, max_value)
            self.ax.set_xlim(0, 5)

            self.fig.canvas.draw()
            self.fig.canvas.flush_events()

    def sensor_dump(self):
        self.drone.sensor_dump()

    def frequency_plot(self):
        p=self.drone.get_frequency_log()
        if self.ax is None:
            plt.ion()
            self.fig, self.ax = plt.subplots()
        self.ax.magnitude_spectrum(p[:-1], Fs=1/p[-1])
        print(p)
        print(len(p))


def start_app():
    app = QApplication([])

    main_window = MainWindow()
    main_window.resize(1000, 600)
    main_window.show()

    app.exec()
