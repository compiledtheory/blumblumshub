import matplotlib.pyplot as plt
from structs import BarPlotInfo, MatrixInfo, ScatterInfo

def bar_plot(info: BarPlotInfo, dir: str) -> None:
    plt.figure(figsize=(10, 6))
    bars = plt.bar(
        info.labels, 
        info.data, 
        color=info.colors)

    plt.title(info.title, fontsize=14)
    plt.ylabel(info.ylabel, fontsize=12)
    plt.ylim(0, info.ymax)
    plt.grid(axis='y', linestyle='--', alpha=0.7)

    for bar in bars:
        yval = bar.get_height()
        plt.text(bar.get_x() + bar.get_width()/2, yval + 1.5, f"{yval:.1f}%", ha='center', va='bottom', fontsize=11)

    plt.tight_layout()
    plt.savefig(f"{dir}/nist_pass_rates.png", dpi=300)
    plt.show()

def cmap_matrix(info: MatrixInfo, dir: str) -> None:
    plt.figure(figsize=(10, 8))
    plt.imshow(info.data, cmap=info.cmap, aspect='auto', interpolation='none')
    plt.title(info.title, fontsize=14, pad=15)
    plt.xlabel(info.xlabel, fontsize=12)
    plt.ylabel(info.ylabel, fontsize=12)

    plt.xticks(ticks=range(len(info.xticks)), labels=info.xticks, rotation=45, ha='right')
    plt.yticks(ticks=range(len(info.data.index)), labels=info.data.index)

    plt.gca().set_xticks([x - 0.5 for x in range(1, len(info.xticks))], minor=True)
    plt.gca().set_yticks([y - 0.5 for y in range(1, len(info.data.index))], minor=True)
    plt.grid(which="minor", color="black", linestyle='-', linewidth=1)

    plt.tight_layout()
    plt.savefig(f"{dir}/pass_fail_matrix.png", dpi=300)
    plt.show()


def scatterplot(info: ScatterInfo, dir: str) -> None:
    plt.figure(figsize=(10, 5))
    plt.scatter(info.x_data, info.y_data, color=info.color, s=60, alpha=0.8, edgecolors='black', linewidth=0.5)
    
    mean_val = info.y_data.mean()
    plt.axhline(mean_val, color='#E07A5F', linestyle='--', alpha=0.7, label=f'Avg: {mean_val:.0f} bits')
    
    plt.title(info.title, fontsize=14)
    plt.xlabel(info.xlabel, fontsize=12)
    plt.ylabel(info.ylabel, fontsize=12)
    plt.ylim(info.ymin, info.ymax)

    plt.xticks(info.x_data)
    plt.grid(axis='both', linestyle='--', alpha=0.4)
    plt.legend()

    plt.tight_layout()
    plt.savefig(f"{dir}/stream_lengths.png", dpi=300)
    plt.show()
