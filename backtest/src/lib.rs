use std::time::Instant;

#[cfg(test)]
mod tests {
    struct SimpleBacktest {
        real_expenses: Vec<f64>,
        nominal_expenses: Vec<f64>,
        stonks: Vec<f64>,
        bonds: Vec<f64>,
        inflation: Vec<f64>,

        stock_fractions: Vec<f64>,
        year_offset: usize,

        i: usize,
        portfolio: f64,
        inflation_factor: f64,
    }

    impl SimpleBacktest {
        fn new(
            start_portfolio: f64,
            real_expenses: Vec<f64>, // real = inflation adjusted, e.g. consumer goods.
            nominal_expenses: Vec<f64>, // nominal = in start year dollars, e.g. mortgage payments.
            stonks: Vec<f64>,
            bonds: Vec<f64>,
            inflation: Vec<f64>,

            stock_fractions: Vec<f64>,
            year_offset: usize,
        ) -> Self {
            assert_eq!(real_expenses.len(), nominal_expenses.len());
            assert_eq!(stock_fractions.len(), real_expenses.len());

            assert_eq!(stonks.len(), super::time_series::YEARS);
            assert_eq!(bonds.len(), super::time_series::YEARS);

            assert!(year_offset < super::time_series::YEARS - stock_fractions.len());

            SimpleBacktest {
                real_expenses,
                nominal_expenses,
                stonks,
                bonds,
                inflation,
                stock_fractions,
                year_offset,
                i: 0,
                portfolio: start_portfolio,
                inflation_factor: 1.0,
            }
        }

        fn portfolio(&self) -> f64 {
            self.portfolio
        }

        fn one_iteration(&mut self) {
            self.portfolio -=
                self.real_expenses[self.i] + self.nominal_expenses[self.i] / self.inflation_factor;

            self.portfolio *= (1.0 + self.stonks[self.year_offset + self.i] / 100.0)
                * self.stock_fractions[self.i]
                + (1.0 + self.bonds[self.year_offset + self.i] / 100.0)
                    * (1.0 - self.stock_fractions[self.i]);

            self.inflation_factor *= 1.0 + self.inflation[self.year_offset + self.i] / 100.0;
            self.i += 1;
        }
    }

    fn compare(
        real_expenses: &[f64],
        nominal_expenses: &[f64],
        stock_fractions: &[f64],
        start_year: usize,
    ) {
        const START_PORTFOLIO: f64 = 1_000.;
        let year_offset = start_year - super::time_series::FIRST_YEAR;

        let mut simple = SimpleBacktest::new(
            START_PORTFOLIO,
            real_expenses.to_vec(),
            nominal_expenses.to_vec(),
            super::time_series::TOTAL_STOCK_MARKET.to_vec(),
            super::time_series::TOTAL_BOND_MARKET.to_vec(),
            super::time_series::INFLATION.to_vec(),
            stock_fractions.to_vec(),
            year_offset,
        );

        for i in 0..stock_fractions.len() {
            let backtest = super::Backtest::new(
                START_PORTFOLIO,
                real_expenses[0..=i].to_vec(),
                nominal_expenses[0..=i].to_vec(),
                super::time_series::TOTAL_STOCK_MARKET.to_vec(),
                super::time_series::TOTAL_BOND_MARKET.to_vec(),
            );

            let actual_end_portfolio =
                backtest.single_run(&stock_fractions[0..=i], year_offset, false);
            simple.one_iteration();
            assert!((actual_end_portfolio - simple.portfolio()).abs() < 1e-3);
        }
    }

    #[test]
    fn real_only() {
        compare(
            &vec![1., 2., 3., 4., 5.],
            &vec![0.; 5],
            &vec![0.4, 0.5, 0.6, 0.7, 0.8],
            1929,
        );
    }

    #[test]
    fn real_and_nominal_only() {
        compare(
            &vec![1., 22., 53., -4., -11.],
            &vec![100., 123., 99., -54., 978.],
            &vec![0.4, 0.5, 0.45, 0.9, 0.23],
            1965,
        );
    }
}

pub mod time_series {
    // From Simba's backtesting spreadsheet
    // https://www.bogleheads.org/wiki/Simba%27s_backtesting_spreadsheet
    // 2021: https://www.bogleheads.org/forum/viewtopic.php?p=5815123#p5815123
    // 2021: https://bit.ly/2NvyAEQ   Tab "Data_Series"

    pub const FIRST_YEAR: usize = 1871;

    pub const YEARS: usize = 2020 - FIRST_YEAR + 1;

    // 1871 - 2020 from "Nominal returns" section of Simba's back testing spreadsheet.
    pub const INFLATION: [f64; YEARS] = [
        1.53, 2.26, -4.41, -6.92, -5.79, 0.88, -15.65, -10.31, 20.69, -5.71, 8.08, -1.87, -7.62,
        -10.31, -3.45, 0.00, 4.76, -4.55, -4.76, 2.50, -6.10, 7.79, -13.25, -4.17, 1.45, -2.86,
        2.94, 1.43, 16.90, -2.41, 2.47, 9.64, -4.40, 2.30, 0.00, 4.49, -2.15, 3.30, 10.64, -6.73,
        -1.03, 7.29, 2.04, 1.00, 2.97, 12.50, 19.66, 17.86, 16.97, -1.55, -11.05, -0.59, 2.98,
        0.00, 3.47, -2.23, -1.14, -1.16, 0.00, -7.02, -10.06, -9.79, 2.33, 3.03, 1.47, 2.17, 2.86,
        -2.78, 0.00, 0.71, 9.93, 9.03, 2.96, 2.30, 2.25, 18.13, 8.84, 2.99, -2.07, 5.93, 6.00,
        0.75, 0.75, -0.74, 0.37, 2.99, 2.90, 1.76, 1.73, 1.36, 0.67, 1.33, 1.64, 0.97, 1.92, 3.46,
        3.04, 4.72, 6.20, 5.57, 3.27, 3.41, 8.71, 12.34, 6.94, 4.86, 6.70, 9.02, 13.29, 12.52,
        8.92, 3.83, 3.79, 3.95, 3.80, 1.10, 4.43, 4.42, 4.65, 6.11, 3.06, 2.90, 2.75, 2.67, 2.54,
        3.32, 1.70, 1.61, 2.68, 3.39, 1.55, 2.38, 1.88, 3.26, 3.42, 2.54, 4.08, 0.09, 2.72, 1.50,
        2.96, 1.74, 1.50, 0.76, 0.73, 2.07, 2.11, 1.91, 2.29, 1.36,
    ];

    pub const LARGE_CAP_BLEND: [f64; YEARS] = [
        13.86, 8.74, 1.99, 12.46, 11.78, -14.95, 16.93, 29.56, 23.75, 34.31, -7.23, 5.54, 2.27,
        -2.34, 34.48, 11.90, -5.18, 8.17, 12.37, -8.48, 26.55, -1.56, -6.43, 8.01, 3.42, 6.21,
        16.89, 27.47, -11.34, 23.89, 16.54, -1.27, -13.31, 29.08, 21.26, -3.67, -22.55, 34.92,
        4.97, 3.55, 4.56, -0.14, -6.68, -6.44, 27.38, -3.83, -31.95, 0.14, 2.23, -12.65, 23.71,
        29.84, 2.37, 27.07, 21.60, 14.07, 34.36, 39.99, -8.54, -21.64, -37.40, 0.49, 46.11, -2.49,
        44.01, 29.05, -36.57, 34.44, -0.42, -10.45, -19.60, 10.15, 21.97, 16.80, 33.26, -22.17,
        -2.98, 2.26, 21.07, 24.04, 16.90, 17.23, -1.72, 53.35, 30.86, 3.36, -13.27, 40.62, 10.01,
        -0.93, 25.98, -9.90, 20.73, 15.27, 10.29, -13.07, 20.18, 5.99, -13.78, -1.63, 10.64, 15.03,
        -21.55, -34.57, 28.28, 18.13, -13.63, -2.89, 4.20, 17.24, -12.97, 16.51, 16.86, 2.18,
        26.43, 16.78, 0.26, 11.30, 25.53, -8.89, 26.35, 4.40, 6.95, -1.46, 34.04, 18.93, 30.96,
        26.58, 17.90, -12.02, -13.32, -23.91, 26.22, 7.33, 1.41, 12.88, 1.33, -37.03, 23.27, 13.35,
        -0.85, 13.98, 30.38, 12.79, 0.63, 9.65, 19.27, -6.22, 28.52, 16.78,
    ];

    // Inflation-adjusted Real Returns, 1871 - 2020
    pub const TOTAL_STOCK_MARKET: [f64; YEARS] = [
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
        -1.83, 14.39, 31.55, 11.71, -0.34, 10.37, 18.67, -6.95, 27.88, 19.36,
    ];

    pub const TOTAL_BOND_MARKET: [f64; YEARS] = [
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
        6.29, 6.27,
    ];

    pub const SHORT_TERM_TREASURIES: [f64; YEARS] = [
        2.15, 2.98, 13.63, 22.10, 11.18, 5.44, 23.18, 18.64, -14.38, 11.72, -3.81, 6.69, 14.91,
        17.31, 8.84, 1.93, -0.01, 11.06, 8.00, 1.59, 14.04, -4.81, 24.35, 9.30, -0.92, 8.75, 2.09,
        2.87, -13.84, 7.27, 0.81, -6.20, 9.68, 3.52, 1.88, -0.93, 5.91, 6.44, -7.88, 13.08, 5.15,
        -4.49, 3.42, 5.23, 1.91, -8.67, -15.25, -10.65, -11.05, 5.11, 24.68, 5.63, 1.21, 6.21,
        -0.77, 6.50, 5.73, 2.86, 5.72, 15.02, 12.01, 19.21, -0.13, -0.24, 0.01, -0.66, -1.77, 4.90,
        1.08, 0.68, -7.87, -7.64, -1.39, -1.09, -0.73, -14.45, -7.28, -1.52, 3.92, -5.00, -4.84,
        1.10, 2.38, 3.24, -0.09, -1.53, 2.55, -0.08, -0.29, 7.18, 1.80, 2.82, 0.76, 2.91, 0.52,
        1.65, 0.61, -0.02, -2.54, 7.53, 4.25, 0.49, -3.78, -4.79, 1.83, 3.13, -2.74, -5.07, -4.81,
        -3.09, 3.52, 16.65, 5.06, 9.31, 9.54, 8.83, 1.17, 1.61, 5.85, 3.31, 8.27, 3.23, 2.48,
        -2.18, 8.04, 1.61, 4.80, 5.25, 0.14, 4.44, 6.66, 3.34, -0.03, -2.34, -1.80, 1.28, 3.03,
        6.50, -1.94, 0.69, -1.46, -1.35, -1.19, -0.25, -0.22, -1.27, -1.68, -0.44, 1.20, 1.70,
    ];

    pub const INTERMEDIATE_TERM_TREASURIES: [f64; YEARS] = [
        1.20, -0.46, 9.05, 22.15, 13.01, 6.24, 22.87, 18.66, -14.72, 11.64, -4.26, 5.03, 12.89,
        15.86, 9.08, 0.75, -3.17, 10.52, 7.62, -1.23, 12.24, -5.28, 20.72, 10.68, -1.40, 7.11,
        2.79, 3.17, -14.80, 6.40, -0.63, -7.94, 6.89, 3.42, 0.64, -3.62, 1.85, 7.84, -8.34, 11.67,
        5.14, -5.34, 1.79, 5.41, 3.17, -7.88, -17.85, -11.22, -12.60, 1.77, 27.22, 6.11, 0.72,
        7.82, -0.30, 7.07, 6.49, 0.23, 4.24, 15.85, 8.85, 24.42, 0.63, 4.09, 2.35, 2.30, -0.96,
        7.29, 3.09, 3.54, -6.99, -7.07, -0.47, -0.01, 2.78, -14.77, -8.56, -0.40, 6.56, -5.74,
        -5.62, 1.42, 2.86, 3.58, -1.18, -4.41, 5.51, -4.50, -3.38, 11.49, 0.62, 4.30, 0.02, 2.83,
        -1.19, 1.44, -3.47, -1.77, -7.85, 11.01, 6.03, -0.28, -4.32, -7.07, -0.41, 9.43, -5.36,
        -7.92, -8.57, -10.68, -2.28, 29.35, 0.42, 10.53, 20.15, 16.14, -3.78, 1.81, 9.97, 2.20,
        13.99, 4.22, 8.32, -7.42, 17.39, -1.38, 7.83, 9.88, -6.52, 10.12, 6.16, 11.34, 0.34, -0.27,
        -1.92, 0.56, 5.67, 14.56, -5.61, 5.38, 6.51, 0.88, -4.18, 3.43, 0.92, -0.96, -0.47, -0.60,
        3.89, 6.20,
    ];

    pub const LONG_TERM_TREASURIES: [f64; YEARS] = [
        5.43, 0.83, 7.93, 17.28, 19.44, 8.04, 25.60, 17.82, -12.40, 14.52, -1.29, 6.40, 11.60,
        16.15, 9.86, 3.79, -5.80, 10.74, 12.53, -2.60, 9.53, -3.20, 16.55, 11.53, 1.67, 6.55, 4.12,
        4.49, -12.08, 6.16, -0.66, -7.07, 4.58, 2.53, 2.49, -4.21, -1.01, 6.23, -7.37, 9.21, 5.03,
        -4.23, 0.54, 3.72, 3.16, -5.31, -20.74, -9.92, -13.71, 1.77, 31.72, 6.85, 1.22, 9.28, 2.69,
        9.70, 9.92, -0.05, 4.85, 14.15, 1.47, 28.88, -2.37, 8.70, 3.22, 5.44, -2.80, 8.37, 4.65,
        7.75, -7.98, -5.43, -0.76, 0.53, 5.92, -14.84, -10.29, 0.59, 9.23, -6.60, -9.57, 0.63,
        1.44, 6.41, -2.47, -8.56, 5.70, -8.36, -6.84, 13.43, -1.24, 4.89, -1.45, 2.77, -2.12, 0.44,
        -10.33, -4.39, -11.34, 7.78, 9.07, 1.42, -7.03, -6.58, 1.23, 11.59, -6.39, -9.66, -12.27,
        -13.80, -7.91, 36.44, -1.81, 10.35, 26.66, 22.66, -6.87, 4.49, 13.58, 0.11, 14.88, 4.84,
        14.03, -10.11, 27.37, -4.13, 13.07, 11.64, -11.19, 16.25, 2.55, 14.00, 0.52, 4.24, 2.91,
        -0.74, 5.43, 23.83, -15.29, 7.78, 25.17, 1.72, -14.03, 24.09, -2.06, -0.76, 6.44, -3.51,
        11.74, 16.11,
    ];

    pub const SHORT_TERM_BONDS: [f64; YEARS] = [
        1.34, 1.77, 12.71, 23.78, 11.26, 5.82, 22.80, 19.04, -14.75, 11.70, -4.12, 6.23, 14.76,
        17.10, 9.10, 1.17, -0.65, 11.24, 7.49, 0.89, 14.20, -5.21, 24.20, 9.83, -1.66, 8.60, 2.47,
        3.02, -14.60, 7.26, 0.40, -6.83, 9.18, 3.90, 1.18, -1.73, 4.80, 7.84, -8.31, 13.03, 5.17,
        -5.03, 3.18, 5.70, 2.41, -8.73, -16.19, -10.81, -11.54, 3.80, 26.13, 5.84, 1.01, 6.93,
        -0.98, 6.58, 5.92, 1.81, 5.53, 15.97, 11.15, 21.12, 0.16, 0.75, 0.50, -0.09, -1.62, 5.50,
        1.45, 1.20, -7.62, -7.72, -0.94, -0.77, -0.04, -14.50, -7.54, -1.20, 4.71, -5.15, -5.18,
        1.13, 2.56, 3.41, -0.52, -2.29, 3.37, -1.21, -0.97, 8.38, 1.62, 3.31, 0.47, 2.93, -0.12,
        1.59, -0.23, -0.44, -4.29, 9.08, 4.78, 0.07, -3.86, -5.30, 1.47, 5.65, -2.96, -5.62, -5.54,
        -4.24, 2.55, 18.98, 4.91, 9.77, 11.22, 10.10, 0.48, 1.72, 6.66, 3.31, 9.70, 3.74, 4.16,
        -3.37, 10.09, 1.19, 5.25, 5.92, -0.59, 5.28, 7.22, 3.69, 1.52, -1.44, -1.97, 1.58, 3.10,
        5.41, 1.62, 2.50, 0.11, 0.30, -1.32, 0.50, 0.19, -0.57, -0.91, -0.55, 2.51, 3.28,
    ];

    pub const INTERMEDIATE_TERM_BONDS: [f64; YEARS] = [
        1.45, -0.84, 8.16, 21.11, 13.56, 6.28, 23.00, 18.42, -14.60, 11.63, -4.20, 4.76, 12.31,
        15.52, 9.00, 0.87, -3.85, 10.22, 7.82, -1.73, 11.47, -5.18, 19.62, 10.81, -1.05, 6.65,
        2.79, 3.17, -14.64, 6.10, -0.85, -8.11, 6.25, 3.12, 0.69, -4.00, 1.21, 7.37, -8.19, 11.20,
        5.12, -5.28, 1.40, 5.15, 3.27, -7.56, -18.13, -11.31, -12.81, 1.49, 27.12, 6.14, 0.69,
        7.90, -0.01, 7.21, 6.62, 0.00, 3.84, 15.49, 8.31, 24.96, 0.70, 4.95, 2.85, 2.99, -0.76,
        7.78, 3.58, 4.22, -6.85, -6.80, -0.45, 0.17, 3.58, -14.85, -8.85, -0.20, 6.98, -5.91,
        -5.66, 1.51, 2.90, 3.58, -1.27, -4.92, 5.98, -5.28, -3.99, 12.17, 0.33, 4.49, -0.05, 2.79,
        -1.36, 1.41, -4.31, -2.09, -8.45, 11.08, 6.25, -0.27, -4.43, -7.48, -0.90, 9.78, -4.32,
        -8.19, -9.06, -8.83, -2.95, 26.78, 3.23, 10.22, 19.94, 15.70, -3.44, 2.86, 9.68, 1.87,
        14.13, 4.80, 9.16, -7.06, 18.04, -0.75, 7.58, 8.35, -5.54, 9.08, 7.62, 8.33, 3.75, 1.98,
        -1.54, 1.40, 3.48, 4.92, 4.06, 7.87, 7.55, 5.19, -4.88, 6.15, 0.54, 0.74, 1.70, -2.04,
        7.72, 8.32,
    ];
}

#[derive(Clone, Copy)]
pub struct Range {
    start: f64,
    end: f64,
    step: f64,
}

impl Range {
    pub fn new(start: f64, end: f64, step: f64) -> Self {
        Range { start, end, step }
    }
}

fn update(ranges: &[Range], values: &mut [f64]) -> bool {
    assert_eq!(ranges.len(), values.len());
    for i in (0..ranges.len()).rev() {
        let this = &ranges[i];

        if values[i] < this.end - this.step / 2. {
            values[i] += this.step;
            if values[i] > this.end {
                values[i] = this.end;
            }

            return false;
        }

        values[i] = this.start;
    }
    return true;
}

pub struct Backtest {
    start_portfolio: f64,
    real_expenses: Vec<f64>, // real = inflation adjusted, e.g. consumer goods.
    nominal_expenses: Vec<f64>, // nominal = in start year dollars, e.g. mortgage payments.
    bond_return: Vec<f64>,
    delta_return: Vec<f64>,
    inflation: Vec<f64>,
}

impl Backtest {
    pub fn new(
        start_portfolio: f64,
        real_expenses: Vec<f64>, // real = inflation adjusted, e.g. consumer goods.
        nominal_expenses: Vec<f64>, // nominal = in start year dollars, e.g. mortgage payments.
        stonks: Vec<f64>,
        bonds: Vec<f64>,
    ) -> Self {
        assert_eq!(real_expenses.len(), nominal_expenses.len());

        assert_eq!(stonks.len(), time_series::YEARS);
        assert_eq!(bonds.len(), time_series::YEARS);

        let mut delta_return: Vec<f64> = Vec::new();
        let mut bond_return: Vec<f64> = Vec::new();
        for i in 0..time_series::YEARS {
            bond_return.push(1.0 + bonds[i] / 100.0);
            delta_return.push((stonks[i] - bonds[i]) / 100.0);
        }

        let mut inflation: Vec<f64> = Vec::new();
        for i in 0..time_series::YEARS {
            inflation.push(1.0 + time_series::INFLATION[i] / 100.0);
        }

        Backtest {
            start_portfolio,
            real_expenses,
            nominal_expenses,
            bond_return,
            delta_return,
            inflation,
        }
    }

    pub fn single_run(&self, stock_fractions: &[f64], year_offset: usize, verbose: bool) -> f64 {
        assert_eq!(stock_fractions.len(), self.real_expenses.len());
        let mut portfolio = self.start_portfolio;

        let mut inflation_factor = 1.0;
        for i in 0..stock_fractions.len() {
            // Remove expenses at the start of the year.
            let expenses = self.nominal_expenses[i] / inflation_factor + self.real_expenses[i];
            portfolio -= expenses;

            // Rebalance, then a year passes.
            portfolio = portfolio
                * (self.bond_return[year_offset + i]
                    + stock_fractions[i] * self.delta_return[year_offset + i]);

            inflation_factor *= self.inflation[year_offset + i];
            if verbose {
                println!(
                    "{}: expenses ${}k, portfolio value ${:.3}k",
                    i + year_offset + time_series::FIRST_YEAR,
                    expenses.round() / 1e3,
                    portfolio.round() / 1e3
                );
            }
        }
        portfolio
    }

    pub fn single_run_general<F: Fn(f64, usize) -> f64>(
        &self,
        f: F,
        year_offset: usize,
        verbose: bool,
    ) -> f64 {
        let mut portfolio = self.start_portfolio;

        let mut inflation_factor = 1.0;
        for i in 0..self.real_expenses.len() {
            // Get stock fraction.
            let years_left = self.real_expenses.len() - i;
            let stock_fraction = f(portfolio, years_left);
            // Remove expenses at the start of the year.
            let expenses = self.nominal_expenses[i] / inflation_factor + self.real_expenses[i];
            let expense_ratio = expenses / portfolio;
            let allowable = 1. / years_left as f64;
            portfolio -= expenses;

            // Rebalance, then a year passes.
            portfolio = portfolio
                * (self.bond_return[year_offset + i]
                    + stock_fraction * self.delta_return[year_offset + i]);

            inflation_factor *= self.inflation[year_offset + i];
            if verbose {
                println!(
                    "{}: expenses ${}k, portfolio value ${:.3}k, stocks: {}%, {:.2}% vs {:.2}%",
                    i + year_offset + time_series::FIRST_YEAR,
                    expenses.round() / 1e3,
                    portfolio.round() / 1e3,
                    (stock_fraction * 1000.).round() / 10.,
                    expense_ratio * 100.,
                    allowable * 100.
                );
            }
        }
        portfolio
    }

    pub fn worst_year(&self, stock_fractions: &Vec<f64>) -> (usize, f64, usize, f64) {
        let mut worst_value = f64::INFINITY;
        let mut worst_year = usize::MAX;

        let mut second_worst_value = f64::INFINITY;
        let mut second_worst_year = usize::MAX;

        for year_offset in 0..(self.bond_return.len() - self.real_expenses.len() + 1) {
            let value = self.single_run(stock_fractions, year_offset, false);
            assert!(value < 1e12);
            if value < worst_value {
                second_worst_year = worst_year;
                second_worst_value = worst_value;
                worst_value = value;
                worst_year = year_offset;
            } else if value < second_worst_value {
                second_worst_value = value;
                second_worst_year = year_offset;
            }
        }
        assert!(worst_year <= time_series::YEARS);
        (
            worst_year + time_series::FIRST_YEAR,
            worst_value,
            second_worst_year + time_series::FIRST_YEAR,
            second_worst_value,
        )
    }

    pub fn worst_year_general<F: Fn(f64, usize) -> f64>(
        &self,
        get_stock_fraction: F,
    ) -> (usize, f64, usize, f64) {
        let mut worst_value = f64::INFINITY;
        let mut worst_year = usize::MAX;

        let mut second_worst_value = f64::INFINITY;
        let mut second_worst_year = usize::MAX;

        for year_offset in 0..(self.bond_return.len() - self.real_expenses.len() + 1) {
            let value = self.single_run_general(&get_stock_fraction, year_offset, false);
            assert!(value < 1e12);
            if value < worst_value {
                second_worst_year = worst_year;
                second_worst_value = worst_value;
                worst_value = value;
                worst_year = year_offset;
            } else if value < second_worst_value {
                second_worst_value = value;
                second_worst_year = year_offset;
            }
        }
        assert!(worst_year <= time_series::YEARS);
        (
            worst_year + time_series::FIRST_YEAR,
            worst_value,
            second_worst_year + time_series::FIRST_YEAR,
            second_worst_value,
        )
    }

    pub fn best_fractions(
        &self,
        ranges: &[Range],
        get_fractions: fn(&[f64], usize) -> Vec<f64>,
        length: usize,
    ) -> (Vec<f64>, f64, usize, f64, usize) {
        let mut values: Vec<f64> = Vec::new();
        // Set all initial values to starts.

        for r in ranges {
            values.push(r.start);
        }

        let mut best_end_portfolio: f64 = f64::NEG_INFINITY;
        let mut best_year: usize = 0;
        let mut best_values: Vec<f64> = Vec::new();
        let mut second_best_end_portfolio: f64 = f64::NEG_INFINITY;
        let mut second_best_year: usize = 0;

        loop {
            let stock_fractions = get_fractions(&values, length);
            let (year, end_portfolio, second_year, second_end_portfolio) =
                self.worst_year(&stock_fractions);
            if end_portfolio > best_end_portfolio {
                best_end_portfolio = end_portfolio;
                best_year = year;
                best_values = values.clone();

                second_best_end_portfolio = second_end_portfolio;
                second_best_year = second_year;
            }

            if update(&ranges, &mut values) {
                break;
            }
        }
        (
            best_values,
            best_end_portfolio,
            best_year,
            second_best_end_portfolio,
            second_best_year,
        )
    }

    pub fn print_fractions(&self, stock_fractions: &[f64]) {
        for f in stock_fractions.iter() {
            print!("{:3.0}% ", f * 100.);
        }
        println!();
    }

    pub fn find_best_ranges(
        &self,
        get_fractions: fn(&[f64], usize) -> Vec<f64>,
        length: usize,
        ranges: &[Range],
    ) {
        let start_time = Instant::now();
        let (
            best_values,
            best_end_portfolio,
            best_year,
            second_best_end_portfolio,
            second_best_year,
        ) = self.best_fractions(ranges, get_fractions, length);
        let elapsed_micros = start_time.elapsed().as_micros();

        print!(
            "elapsed: {:.1} msec: {:.0}%",
            elapsed_micros as f64 / 1000.,
            best_values[0] * 100.0
        );
        for i in 1..best_values.len() {
            print!(" to {:.0}%", best_values[i] * 100.0);
        }

        println!(
            ", ${:.3} M, worst year starting {} (${:.3} M, second worst year {})",
            best_end_portfolio / 1e6,
            best_year,
            second_best_end_portfolio / 1e6,
            second_best_year,
        );

        self.print_fractions(&get_fractions(&best_values, length));
    }
}
