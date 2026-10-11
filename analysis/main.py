import os
from pathlib import Path
import pandas as pd
import graphs
import structs
from matplotlib.colors import ListedColormap

IN_PATH = Path(r"blumblumshub/data/results.csv")
OUT_PATH = "blumblumshub/analysis/output"
TEST_RESULTS = ["monobit", "block", "runs", "longest_ones", "serial_1", "serial_2"]

def main(path: Path) -> None:
    os.makedirs(OUT_PATH, exist_ok=True)
    data = pd.read_csv(path)
    test_info = data[TEST_RESULTS]

    bar_plot_info = structs.BarPlotInfo(
        labels=TEST_RESULTS,
        data=test_info.mean() * 100,
        colors=['#E07A5F', '#F2CC8F', '#E8D595', '#81B29A', '#3D405B', '#9A8C98'],
        title=f'NIST SP 800-22 Test Pass Rates ({len(data.index)} Streams)',
        ylabel='Pass Rate (%)',
        ymax=110
    )

    graphs.bar_plot(bar_plot_info, OUT_PATH)

    matrix_info = structs.MatrixInfo(
        data = data.set_index("stream_id")[TEST_RESULTS],
        cmap=ListedColormap(['#E06666', '#7BCA8F']),
        title='NIST Tests Pass/Fail Matrix',
        xlabel='NIST Tests',
        ylabel='Stream ID',
        xticks=TEST_RESULTS,
    )

    graphs.cmap_matrix(matrix_info, OUT_PATH)

    scatter_info = structs.ScatterInfo(
            x_data=data['stream_id'],
            y_data=data['bits'],
            color='#3D405B',
            title='Length of generated BBS streams',
            xlabel='Stream ID',
            ylabel='Bits',
            ymin=95000,
            ymax=115000
        )

    graphs.scatterplot(scatter_info, OUT_PATH)


if __name__ == "__main__":
    main(IN_PATH)