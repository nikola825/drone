from PySide6.QtWidgets import QApplication, QWidget, QVBoxLayout, QHBoxLayout, QGridLayout, QPushButton, QSizePolicy, \
    QLabel, QSpinBox, QCheckBox, QDial, QSlider, QLCDNumber, QProgressBar
from PySide6.QtCore import Qt, QTimer, Slot

from dronecontrol import COMMAND_INPUT_MAX, THRUST_MAX


class MainWindow(QWidget):
    def __init__(self):
        super().__init__()
        self.expand_policy = QSizePolicy(QSizePolicy.Policy.Expanding, QSizePolicy.Policy.Expanding)
        self.setup_layout()
        self.down_keys = set()
        self.setup_timer()

    def keyPressEvent(self, event):
        self.down_keys.add(event.key())

    def keyReleaseEvent(self, event):
        self.down_keys.remove(event.key())

    def setup_layout(self):
        self.setup_base_layout()
        self.setup_top_menu()
        self.setup_status_grid()
        self.setup_property_list()

    def setup_base_layout(self):
        self.base_layout = QGridLayout(self)
        self.layout = self.base_layout
        self.top_menu = QHBoxLayout()
        self.status_grid = QGridLayout()
        self.property_list = QGridLayout()

        self.fill(self.base_layout, 1, 0, self.expand_policy)
        self.fill(self.base_layout, 1, 1, self.expand_policy)

        self.base_layout.addLayout(self.top_menu, 0, 0, 1, -1)
        self.base_layout.addLayout(self.status_grid, 1, 0)
        self.base_layout.addLayout(self.property_list, 1, 1)

    def setup_top_menu(self):
        top_menu_buttons = [
            (QPushButton(text="Connect"), None),
            (QPushButton(text="Start"), None),
            (QPushButton(text="Stop"), None)
        ]
        for button, callback in top_menu_buttons:
            self.top_menu.addWidget(button)

    def setup_status_grid(self):
        status_grid_rows = 7
        status_grid_columns = 7
        add_buttons = True

        for row in range(status_grid_rows):
            for column in range(status_grid_columns):
                self.fill(self.status_grid, row, column, self.expand_policy)

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

    def setup_property_list(self):
        properties = ["Yaw", "Pitch", "Rol"]
        row_count = 0
        for prop in properties:
            label = QLabel(text=prop)
            box = QSpinBox()
            periodic_refresh = QCheckBox(text="Auto refresh")
            apply_button = QPushButton(text="Apply")
            refresh_button = QPushButton(text="Refresh")

            self.property_list.addWidget(label, row_count, 0)
            self.property_list.addWidget(box, row_count, 1)
            self.property_list.addWidget(periodic_refresh, row_count, 2)
            self.property_list.addWidget(apply_button, row_count, 3)
            self.property_list.addWidget(refresh_button, row_count, 4)

            row_count += 1

    def setup_timer(self):
        self.periodic_timer = QTimer()
        self.periodic_timer.setInterval(50)
        self.periodic_timer.timeout.connect(self.timer_tick)
        self.periodic_timer.start()

    @Slot()
    def timer_tick(self):
        pass

    def get_filler(self, size_policy):
        dummy_label = QLabel()
        dummy_label.setSizePolicy(size_policy)
        return dummy_label

    def fill(self, grid, row, column, size_policy):
        filler = self.get_filler(size_policy)
        grid.addWidget(filler, row, column)


def start_app():
    app = QApplication([])

    mainWindow = MainWindow()
    mainWindow.resize(1000, 600)
    mainWindow.show()

    app.exec()
