# Code written Saturday Jan 29, 2022 for the Bogleheads forum post "The Great
# Fixed Vs Bond Tent Blow Out"
#
# https://bogleheads.org/forum/viewtopic.php?p=6484507#p6484507

import matplotlib.pyplot as plt

FIRST_YEAR_OF_MARKET_RETURNS = 1871

# From Simba's backtesting spreadsheet
# https://www.bogleheads.org/wiki/Simba%27s_backtesting_spreadsheet
# 2021: https://www.bogleheads.org/forum/viewtopic.php?p=5815123#p5815123
# 2021: https://bit.ly/2NvyAEQ   Tab "Data_Series"

# Notes on cFIREsim's data:
#
# The annual data is in js/marketData.js.  It is *not* inflation adjusted.
#
# https://github.com/boknows/cFIREsim-open/blob/master/js/marketData.js

# Inflation-adjusted Real Returns, 1871 - 2020
# fmt: off
TOTAL_STOCK_MARKET = [
    13.86, 8.74, 1.99, 12.46, 11.78, -14.95, 16.93, 29.56, 23.75, 34.31, -7.23, 5.54, 2.27,
    -2.34, 34.48, 11.90, -5.18, 8.17, 12.37, -8.48, 26.55, -1.56, -6.43, 8.01, 3.42, 6.21,
    16.89, 27.47, -11.34, 23.89, 16.54, -1.27, -13.31, 29.08, 21.26, -3.67, -22.55, 34.92,
    4.97, 3.55, 4.56, -0.14, -6.68, -6.44, 27.38, -3.83, -31.95, 0.14, 2.23, -12.65, 23.71,
    29.84, 2.37, 27.07, 21.60, 14.07, 34.88, 40.32, -11.29, -22.94, -37.87, 1.26, 53.62, -0.70,
    44.18, 30.79, -36.66, 32.06, 2.35, -7.83, -18.23, 7.04, 24.23, 18.50, 35.19, -20.66, -4.59,
    -0.60, 22.53, 23.03, 13.91, 12.80, -0.09, 51.70, 25.26, 5.36, -12.46, 42.32, 10.86, -0.43,
    26.08, -10.79, 19.20, 15.29, 12.12, -11.81, 23.59, 8.09, -15.51, -4.43, 13.91, 14.05,
    -25.07, -36.28, 29.44, 20.67, -8.79, 0.20, 10.78, 18.75, -11.67, 14.29, 18.91, -0.91,
    27.66, 14.79, -2.11, 12.90, 23.39, -11.62, 30.16, 5.85, 7.67, -2.77, 32.42, 17.07, 28.80,
    21.31, 20.58, -13.49, -12.25, -22.79, 28.99, 9.06, 2.58, 12.76, 1.43, -37.05, 25.41, 15.53,
    -1.83, 14.39, 31.55, 11.71, -0.34, 10.37, 18.67, -6.95, 27.88, 19.36, 17.45,
]

TOTAL_BOND_MARKET = [
    1.22, -0.44, 9.08, 22.18, 13.03, 6.26, 22.89, 18.68, -14.70, 11.66, -4.24, 5.05, 12.91,
    15.89, 9.10, 0.77, -3.15, 10.54, 7.64, -1.21, 12.26, -5.27, 20.75, 10.71, -1.38, 7.14,
    2.82, 3.19, -14.78, 6.42, -0.61, -7.92, 6.91, 3.44, 0.66, -3.60, 1.87, 7.86, -8.32, 11.69,
    5.16, -5.32, 1.81, 5.43, 3.19, -7.86, -17.84, -11.21, -12.59, 1.79, 27.25, 6.13, 0.74,
    7.84, -0.28, 7.09, 6.51, 0.25, 4.26, 15.87, 8.88, 24.44, 0.65, 4.11, 2.37, 2.32, -0.94,
    7.32, 3.11, 3.56, -6.98, -7.05, -0.45, 0.01, 2.80, -14.75, -8.54, -0.38, 6.58, -5.72,
    -5.60, 1.44, 2.88, 3.60, -1.16, -4.39, 5.53, -4.48, -3.36, 11.51, 0.64, 4.32, 0.04, 2.85,
    -1.17, 1.46, -3.45, -1.75, -7.83, 11.03, 6.05, -0.26, -4.83, -4.75, 1.01, 10.18, -3.48,
    -7.04, -10.08, -8.77, -2.50, 27.67, 4.35, 10.72, 17.58, 13.96, -2.78, 2.81, 8.60, 2.40,
    11.82, 4.12, 6.75, -5.19, 15.26, 0.25, 7.61, 6.86, -3.35, 7.74, 6.78, 5.80, 2.12, 1.04,
    -0.90, 1.78, 2.82, 5.05, 3.23, 4.97, 4.59, 2.37, -3.59, 5.10, -0.33, 0.51, 1.42, -1.90,
    6.29, 6.27, -8.13,
]
# fmt: on

YEARS_OF_MARKET_RETURNS = len(TOTAL_STOCK_MARKET)

LENGTH = 30
INITIAL_PORTFOLIO = 1_000_000
WITHDRAWL_RATE = 0.04
EXPENSES = [INITIAL_PORTFOLIO * WITHDRAWL_RATE] * LENGTH

# 20 years: 60% works best.
# 30 years: 68% works best.
FIXED_AA_STOCKS = 0.70
# 20 years: 63% to 97% over 14 years works best.
# 30 years: 47% to 100% over 14 years works best.
#
# So, bond tents to consider:
#     50% -> 100% over 15 years
#     50% ->  70% over 15 years
BOND_TENT_START_STOCKS = 0.5
BOND_TENT_END_STOCKS = 1.0
BOND_TENT_YEARS = 15


FIXED_AA = [FIXED_AA_STOCKS] * LENGTH
BOND_TENT = []
for year in range(LENGTH):
    if year < BOND_TENT_YEARS:
        BOND_TENT.append(
            BOND_TENT_START_STOCKS
            + (BOND_TENT_END_STOCKS - BOND_TENT_START_STOCKS) / BOND_TENT_YEARS * year
        )
    else:
        BOND_TENT.append(BOND_TENT_END_STOCKS)


def retirement(
    start_year,
    stock_fractions,
    expenses=EXPENSES,
    initial_portfolio=INITIAL_PORTFOLIO,
    stocks=TOTAL_STOCK_MARKET,
    bonds=TOTAL_BOND_MARKET,
):
    assert len(expenses) == len(stock_fractions)
    for fraction in stock_fractions:
        assert fraction >= 0 and fraction <= 1
    portfolio = initial_portfolio
    nest_egg = []
    for year in range(start_year, start_year + len(stock_fractions)):
        # First, we withdraw expenses
        portfolio -= expenses[year - start_year]
        # Next, we rebalance & simulate a year of returns.
        stock_f = stock_fractions[year - start_year]
        portfolio *= stock_f * (
            1.0 + stocks[year - FIRST_YEAR_OF_MARKET_RETURNS] / 100.0
        ) + (1 - stock_f) * (1.0 + bonds[year - FIRST_YEAR_OF_MARKET_RETURNS] / 100.0)
        # Record the portfolio value at the end of the year.
        nest_egg.append(portfolio)
    return nest_egg


start_years = list(
    range(
        FIRST_YEAR_OF_MARKET_RETURNS,
        FIRST_YEAR_OF_MARKET_RETURNS + YEARS_OF_MARKET_RETURNS - LENGTH + 1,
    )
)


def end_values(stock_fractions):
    result = []
    for start_year in start_years:
        result.append(retirement(start_year, stock_fractions)[-1])
    return result


tent_end_values = end_values(BOND_TENT)
fixed_end_values = end_values(FIXED_AA)

if False:
    for stocks in range(10, 101):
        fixed_ends = end_values([stocks / 100] * LENGTH)
        # print([fixed_ends[i] < tent_end_values[i] for i in range(len(fixed_ends))])
        num_tent_better = [
            fixed_ends[i] < tent_end_values[i] for i in range(len(fixed_ends))
        ].count(True)
        print(f"{stocks}%: {num_tent_better} tent wins")

width = 0.37
colors = [
    "blue" if fixed_end_values[i] < tent_end_values[i] else "orange"
    for i in range(len(fixed_end_values))
]


def restrict(target, start_years, min_year, max_year):
    assert len(target) == len(start_years)
    result = []
    for i in range(len(start_years)):
        if start_years[i] >= min_year and start_years[i] <= max_year:
            result.append(target[i])
    return result


def lowest(values, years):
    assert len(values) == len(years)
    low = values[0]
    low_year = years[0]
    for i in range(len(values)):
        if values[i] < low:
            low = values[i]
            low_year = years[i]
    return (low, low_year)


# Blue: Tent better
# Orange: Fixed better
def plot(min_year, max_year, num_xticks, fname):
    # Using numpy for this would be much simpler.  But then the reader would have
    # to know numpy.  So stick with pure Python.
    start_years_range = restrict(start_years, start_years, min_year, max_year)
    fixed_end_values_range = restrict(fixed_end_values, start_years, min_year, max_year)
    tent_end_values_range = restrict(tent_end_values, start_years, min_year, max_year)
    # colors_range = restrict(colors, start_years, min_year, max_year)

    fig, ax = plt.subplots()
    ax.bar(
        [y - width / 2 for y in start_years_range],
        [v / 1000 for v in fixed_end_values_range],
        width,
        label=f"Fixed {FIXED_AA_STOCKS*100:g}%/{(1.0-FIXED_AA_STOCKS)*100:g}%",
        color="orange",
        # color=colors_range,
    )
    ax.bar(
        [y + width / 2 for y in start_years_range],
        [v / 1000 for v in tent_end_values_range],
        width,
        label=f"Bond Tent {BOND_TENT_START_STOCKS*100:g}% -> {BOND_TENT_END_STOCKS*100:g}% Stocks over {BOND_TENT_YEARS} Years",
        color="blue",
        # color=colors_range,
    )
    ax.locator_params(axis="x", nbins=num_xticks)
    ax.set_ylabel(f"Ending portfolio value after {LENGTH} years, $ thousands")
    ax.legend()
    ax.set_title(
        f"Portfolio value after {LENGTH} years, starting retirement {min_year}-{max_year}"
    )
    print(
        f"Portfolio value after {LENGTH} years, starting retirement {min_year}-{max_year}"
    )
    print(
        f"  fixed {FIXED_AA_STOCKS*100:g}%: {lowest(fixed_end_values_range, start_years_range)}"
    )
    print(
        f"{BOND_TENT_START_STOCKS*100:g}% -> {BOND_TENT_END_STOCKS*100:g}%: {lowest(tent_end_values_range, start_years_range)} over {BOND_TENT_YEARS} years"
    )

    plt.savefig(fname)
    plt.show()


def imgfname(prefix):
    return f"{prefix}_{FIXED_AA_STOCKS}_{BOND_TENT_START_STOCKS}_{BOND_TENT_END_STOCKS}_{BOND_TENT_YEARS}_{LENGTH}.png"


plot(1955, 1974, 8, imgfname("stagflation"))
plot(1925, 1945, 6, imgfname("great_depression"))
plot(1900, 1918, 6, imgfname("panic_of_1907"))
if LENGTH <= 20:
    plot(1990, 2020, 3, imgfname("dot_com_crash"))
