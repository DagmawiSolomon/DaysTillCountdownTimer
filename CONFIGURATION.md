# Configuration Guide for Days Till Counter

The application can be customized using a `config.json` file. Below is an example configuration and explanations for each field.

## Example Configuration

```json
{
    "Windowsettings": {
      "Resizable": false,
      "MarginBottom": 10,
      "MarginEnd": 15,
      "MarginStart": 15
    },
    "Datetime": {
      "Startdate": {
        "Year": 2024,
        "Month": 5,
        "Day": 24
      },
      "Enddate": {
        "Year": 2025,
        "Month": 1,
        "Day": 1
      }  
    },
    "Widgets": {
      "WidgetType": "Grid",
      "Label": {
        "Text": "Deadline",
        "Alignment": "start"
      },
      "Grid": {
        "Width": 12,
        "Height": 12,
        "RowSpacing": 5,
        "RowWidth": 15,
        "ColumnSpacing": 5,
        "DaysPassedColor": {
          "R": 45,
          "G": 186,
          "B": 78
        },
        "DaysLeftColor": {
          "R": 36,
          "G": 41,
          "B": 46
        }
      }
    }
}
```

## Configuration Fields

### Windowsettings

- `Resizable`: Specifies whether the window can be resized. (`true` or `false`)
- `MarginBottom`: The bottom margin of the window in pixels.
- `MarginEnd`: The right margin of the window in pixels.
- `MarginStart`: The left margin of the window in pixels.

### Datetime

- `Startdate`: The starting date for the countdown.
  - `Year`: The year to start from.
  - `Month`: The month to start from.
  - `Day`: The day to start from.
- `Enddate`: The target date for the countdown.
  - `Year`: The target year.
  - `Month`: The target month.
  - `Day`: The target day.

### Widgets

- `WidgetType`: The type of widget to use. Currently supports `"Grid"` and `"ProgressBar"`.
- `Label`: Settings for the label displayed in the widget.
  - `Text`: The text to display.
  - `Alignment`: The alignment of the text (`"start"`, `"center"`, `"end"`).
- `Grid`: Configuration for the grid widget.
  - `Width`: The number of columns in the grid.
  - `Height`: The number of rows in the grid.
  - `RowSpacing`: The spacing between rows in pixels.
  - `RowWidth`: The width of each row in pixels.
  - `ColumnSpacing`: The spacing between columns in pixels.
  - `DaysPassedColor`: The color for days that have passed.
    - `R`: Red component (0-255).
    - `G`: Green component (0-255).
    - `B`: Blue component (0-255).
  - `DaysLeftColor`: The color for days that are left.
    - `R`: Red component (0-255).
    - `G`: Green component (0-255).
    - `B`: Blue component (0-255).
