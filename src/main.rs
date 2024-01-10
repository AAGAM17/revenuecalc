use slint::SharedString;

slint::include_modules!();

const OWNERPER: f64 = 0.55;
const PROFITPER: f64 = 0.05;
const OPEXPER: f64 = 0.10;

fn calculate_tax(num: f64, country: &str) -> f64 {
    if country == "USA" {
        if num <= 11000.0 {
            num * 0.10
        } else if num <= 44725.0 {
            num * 0.12
        } else if num <= 95375.0 {
            num * 0.22
        } else if num <= 182100.0 {
            num * 0.24
        } else if num <= 231250.0 {
            num * 0.32
        } else if num <= 578125.0 {
            num * 0.35
        } else {
            num * 0.37
        }
    } else if country == "India" {
        if num <= 250000.0 {
            0.0
        } else if num <= 500000.0 {
            num * 0.05
        } else if num <= 750000.0 {
            num * 0.10
        } else if num <= 1000000.0 {
            num * 0.15
        } else if num <= 1250000.0 {
            num * 0.20
        } else if num <= 1500000.0 {
            num * 0.25
        } else {
            num * 0.30
        }
    } else {
        num * 0.30
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();
    ui.on_divide_income(move |string: SharedString| {
        let ui: AppWindow = ui_handle.unwrap();
        let num: f64 = string.trim().parse().unwrap();
        let country = "USA";
        let tax: f64 = calculate_tax(num, country);
        let owner: f64 = num * OWNERPER;
        let profit: f64 = num * PROFITPER;
        let opex: f64 = num * OPEXPER;
        let result: String = format!(
            "Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOpEx: {:.2}",
            {tax}, {owner}, {profit}, {opex}
        );
        ui.set_results(result.into());
    });

    ui.run()
}