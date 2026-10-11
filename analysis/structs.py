from dataclasses import dataclass

from pandas import Series, DataFrame
from matplotlib.colors import ListedColormap


@dataclass
class BarPlotInfo:
    labels: list[str]
    data: Series
    colors: list[str]
    title: str
    ylabel: str
    ymax: int

@dataclass 
class MatrixInfo:
    data: DataFrame
    cmap: ListedColormap
    title: str
    xlabel: str
    ylabel: str
    xticks: list[str]

@dataclass
class ScatterInfo:
    x_data: Series
    y_data: Series
    color: str
    title: str
    xlabel: str
    ylabel: str
    ymin: int
    ymax: int
