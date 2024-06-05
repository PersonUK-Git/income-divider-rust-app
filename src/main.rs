use std::result;

slint::include_modules!();

const TAXPER: f64 = 0.30;
const OWNERPER: f64 = 0.55;
const PROFITPER : f64 = 0.05;
const OPEXPER : f64 = 0.10; 
fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_Divide_income({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();

            let num : f64 = string.trim().parse().unwrap();
            let tax  : f64 = num * TAXPER;
            let owner  : f64 = num * OWNERPER;
            let profit  : f64 = num * PROFITPER;
            let opex  : f64 = num * OPEXPER;
            let result: String  = format!("Tax: {:.2},\n Owner: {:.2},\n Profit: {:.2},\n OpEx: {:.2}\n", tax, owner, profit, opex);
            ui.set_results(result.into());
        }
    });

    ui.run()
}
