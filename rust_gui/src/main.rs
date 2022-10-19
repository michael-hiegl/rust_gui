//Imports############################################################################################
use eframe::run_native;
use eframe::egui;
use eframe::App;
//###################################################################################################

fn main()
{
    run_native("Application", eframe::NativeOptions::default(), Box::new(|_cc| Box::new(Application::default())));
}

struct Application
{
    counter: i32,
}

impl Default for Application
{
    fn default() -> Self
    {
        Self
        {
            counter: 10,
        }
    }
}

impl App for Application
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        //Applicationcode############################################################################
        egui::CentralPanel::default().show(ctx, |ui|
        {
            let mut counter: i32 = 10;
            ui.label(format!("Hello i am {0}", self.counter));
            if ui.button("+").clicked()
            {
                self.counter+=1;
            }
        });
        //###########################################################################################
    }  
}