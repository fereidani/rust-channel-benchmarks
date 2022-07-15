#!/usr/bin/env python3
import sys
import pygal


def read_data(files):
    benchs = {}
    # only to keep list shorted
    names = ["kanal", "kanal-async",
             "crossbeam-channel", "flume", "flume-async", "std::mpsc", "futures-channel"]
    for f in files:
        with open(f) as f:
            lines = f.readlines()
            name = lines[0].strip("\n ")
            if name not in names:
                names.append(name)
            for line in lines[1:]:
                test, nsecs = line.strip("\n ").split(",")
                splt = test.split('_')
                test_cat = splt[0]
                test_name = '_'.join(splt[1:])
                if test_cat not in benchs:
                    benchs[test_cat] = {}
                if test_name not in benchs[test_cat]:
                    benchs[test_cat][test_name] = {}
                benchs[test_cat][test_name][name] = float(nsecs)

    return benchs, names


labels = ["seq", "spsc", "mpsc", "mpmc"]  # "select_rx", "select_both"


titles = {
    "unbounded": "Unbounded Channel Benchmark\n(Relative time, lower is better)",
    "bounded": "Bounded Channel With Size N Benchmark\n(Relative time, lower is better)",
    "bounded0": "Bounded Channel With Size 0 Benchmark\n(Relative time, lower is better)",
    "bounded1": "Bounded Channel With Size 1 Benchmark\n(Relative time, lower is better)",
}


def is_all_none(arr):
    for v in arr:
        if v is not None:
            return False
    return True


color_set = {
    'magenta': '#ff00ff',
    'maroon': '#800000',
    'navy': '#000080',
    'olive': '#808000',
    'orange': '#ffa500',
    'purple': '#800080',
    'lightpurple': '#eb50eb',
    'red': '#ff0000',
    'aqua': '#00ffff',
    'black': '#000000',
    'blue': '#0000ff',
    'brown': '#301607',
    'cyan': '#00ffff',
    'darkblue': '#00008b',
    'darkcyan': '#008b8b',
    'darkgrey': '#a9a9a9',
    'darkgreen': '#006400',
    'darkkhaki': '#bdb76b',
    'darkmagenta': '#8b008b',
    'darkolivegreen': '#556b2f',
    'darkorange': '#ff8c00',
    'darkorchid': '#9932cc',
    'darkred': '#8b0000',
    'darksalmon': '#e9967a',
    'darkviolet': '#9400d3',
    'fuchsia': '#ff00ff',
    'gold': '#ffd700',
    'azure': '#19ffae',
    'indigo': '#4b0082',
    'khaki': '#f0e68c',
    'lime': '#00ff00',
    'lightblue': "#3dc2ff",
    'lightbrown': "#7a4d32",
    'green': "#0aa16b"
}


colors = {
    "kanal": "green",
    "kanal-async": "azure",
    "go": "blue",
    "flume": "purple",
    "flume-async": "lightpurple",
    "async-channel": "lightblue",
    "crossbeam-channel": "red",
    "std::mpsc": "brown",
    "futures-channel": "lightbrown",
}


def get_color(name):
    if name.find("go") == 0:
        name = "go"
    if name in colors:
        return color_set[colors[name]]
    reserved = sorted(colors.values())
    for k in color_set:
        if k not in reserved:
            colors[name] = k
            print(color_set[k])
            return color_set[k]


def make_rows(bench_name, bench, names):
    bench_keys = sorted(bench.keys())
    x_labels = []
    for label in labels:
        if label in bench_keys:
            x_labels.append(label)

    print(bench_name, x_labels)
    rows = []
    for name in names:
        row = []
        for label in x_labels:
            if name in bench[label]:
                # row.append(round(bench[label][name]/1e9, 2)) # convert to seconds
                row.append(bench[label][name])
            else:
                row.append(None)

        # if not is_all_none(row):
        #    rows.append((name, row))
        #    print(name, row)
        rows.append((name, row))
    print(rows)
    return rows, x_labels
    # print(bench)


def normalize_rows(rows):
    norm = []
    for (name, row) in rows:
        if name == "kanal":
            norm = row
    for idx, (name, row) in enumerate(rows):
        rows[idx] = (
            name, [round(i / j, 2) if i is not None else 0 for i, j in zip(row, norm)])


def chart(benchs, names):
    for bench_name in benchs:
        bench = benchs[bench_name]
        rows, label_list = make_rows(bench_name, bench, names)
        x_labels = []
        for label in labels:
            if label in label_list:
                x_labels.append(label)
        bar_colors = []
        for (name, _) in rows:
            bar_colors.append(get_color(name))
        custom_style = pygal.style.Style(
            colors=bar_colors, value_font_size=9,
            value_colors=bar_colors,
        )
        chart = pygal.Bar(
            legend_at_bottom=True,
            print_values=True,
            print_values_position='top',
            value_formatter=lambda x: '{}x'.format(
                x) if x > 0 else "N/A*",
            style=custom_style)
        chart.title = titles[bench_name]
        chart.x_labels = x_labels
        normalize_rows(rows)
        for (name, row) in rows:
            chart.add(name, row)
        chart.render_to_png("target/plot_{}.png".format(bench_name))


def main():
    benchs, names = read_data(sys.argv[1:])
    chart(benchs, names)


if __name__ == '__main__':
    main()
