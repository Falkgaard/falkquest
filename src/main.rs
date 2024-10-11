use web_time::{Duration, Instant};
use chrono::{DateTime, Local};

use eframe::egui;

type Id = String;

#[derive(serde::Deserialize, serde::Serialize)]
struct Attributes {
    toughness: f32,
    endurance: f32,
    dexterity: f32,
    intellect: f32,
    dominance: f32,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LifePool {
   cur: u16,
   max: u16,
}

impl LifePool {
    fn set_max( &mut self, val: u16) {
        if val == 0 {
            panic!("Max life may not be 0!");
        }
        let ratio = (self.cur as f32) / (self.max as f32);
        self.max  = 1.max( val );
        self.cur  = (ratio * self.max as f32) as u16;
    }
}



/*
enum WeaponType {
    MainHand,
    OffHand,
    TwoHand,
}

enum WornType {
    Headwear,
    Chestwear,
    Underwear,
    Legwear,
    Footwear,
    Handwear,
    Backwear,
}

enum EquipmentType {
    Weapon( type: WeaponType ),
}

enum ObjTag {
   ForInventory,
   ForBase,
   Equipment,
   Container( max_slots: u8, cur_slots: u8, max_size: ObjSize ),
   Static,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum ObjSize {
    Tiny,
    Small,
    Medium,
    Large,
    Massive,
}

struct Obj {
    id        : u16, // Str?
    name      : String,
    desc      : String,
    tags      : Vec<ObjTag>,
    amount    : u64,
    max_stack : Option<u64>,
    size      : ItemSize,
    weight    : f32,
}

enum WearSlotTag {
    FeetOuter,
    FeetInner,
    LegsOuter,
    LegsInner,
    ChestInner,
    ChestOuter,
    Body,
    Hands,
    Ring,
    Neck,
    Face,
    Eyes,
    Head,
    Back,

}

struct WornItems {
    feet   : Option<Obj>, // Sandals, shoes, etc
    socks  : Option<Obj>, // Socks, stockings, etc
    legs   : Option<Obj>, // Pants, skirts, etc
    crotch : Option<Obj>, // Underwear
    chest  : Option<Obj>, // Shirts
    body   : Option<Obj>, // Jackets, coats, etc
    hands  : Option<Obj>, // Gloves
    ring_1 : Option<Obj>,
    ring_2 : Option<Obj>,
    neck   : Option<Obj>, // Collars, necklaces, etc
    face   : Option<Obj>, // Masks
    eyes   : Option<Obj>, // Shades, eyepatches, visors, etc
    head   : Option<Obj>, // Hats
    back   : Option<Obj>, // Backpacks, jetpacks, etc
}

struct HeldItems {
    main_hand  : Option<Obj>,
    off_hand   : Option<Obj>,
    both_hands : Option<Obj>,
}

Containers:

Hands

fn try_pick_up( &mut self, obj: Obj ) -> Option<Obj> {
    self.slots.filter( |x| x.tags.contains
   for slot in self.slots {
        if slot.i
   }
}


*/


#[derive(serde::Deserialize, serde::Serialize)]
struct Actor {
    id         : Id,
    name       : String,
    attributes : Attributes,
    life       : LifePool,
}

#[derive(serde::Deserialize, serde::Serialize)]
enum State {
    Intro { progress: u8 },
    Gaming,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
struct IdleApp {
    player       : Actor,
    elapsed_secs : f32,
    time_factor  : f32,
    state        : State,
}

impl Default for IdleApp {
    fn default() -> Self {
        Self {
            player : Actor {
                id         : "player".to_string(),
                name       : "Falk".to_string(),
                life       : LifePool { cur: 1, max: 5 },
                attributes : Attributes {
                    toughness: 1.0,
                    endurance: 1.0,
                    dexterity: 1.0,
                    intellect: 1.0,
                    dominance: 1.0,
                },
            },
            elapsed_secs : 0.0,
            time_factor  : 1.0,
            state        : State::Intro { progress: 0 },
        }
    }
}

impl IdleApp {
    pub fn new( cc: &eframe::CreationContext<'_> ) -> Self {
        // Load previous app state (if any).
        // TEMP: if let Some(storage) = cc.storage {
        // TEMP:     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // TEMP: }
        Default::default()
    }
}

enum TabTag {
    Character,
    Inventory,
    Base,
    Cult,
    Factions,
    Objectives,
    Actions,
}


enum LogEntry{
    Info { timestamp: DateTime<Local>, text: String },
}


trait Action {
    fn meets_requirements( &self, state: &State ) -> bool;
    fn perform( &mut self, state: &mut State )    -> Result<(),&str>;
}



/*
    "You look for meth-heads to recruit, but you've already recruited the local population..."
    " wait..."
    " you get a brilliant idea! 💡\n"
    "[You have unlocked the action "Cultivate Meth-Heads"]

    Action {
        name     : "Cultivate Meth-Heads",
        desc     : "Lace candy with meth and hand it out at pre-school playgrounds.",
        consumes : [{2,Candy}, {1,Meth}],
        produces : [{1,LocalMethHead}]
    }
*/

fn intro_update( state: &mut IdleApp, ctx: &egui::Context/*, _frame: &mut eframe::Frame*/ ) {
    egui::CentralPanel::default().show( ctx, |ui| {
        ui.label("The camera fades in on the grimy streets of a run-down city block. It's the kind of place where even the rats avoid eye contact, and the smell is a cross between burnt rubber and shattered dreams. Nestled between two rusting dumpsters in a back alley, Falk stirs, groaning. His face is half-submerged in a slick puddle of what is unmistakably last night's poor choices.");

        if state.elapsed_secs >= 8.0 {
            ui.label("");
            if   state.elapsed_secs < 10.0 { ui.label("Falk: \"Ugh...\""); }
            else                           { ui.label("Falk: \"Ugh... where the hell am I?\" His voice croaks, like a chainsmoker gargling gravel."); }
        }

        if state.elapsed_secs >= 14.0 {
            ui.label("");
            if      state.elapsed_secs < 14.0 { ui.label("\"Wait...\""); }
            else if state.elapsed_secs < 17.0 { ui.label("\"Wait... who...\""); }
            else                              { ui.label("\"Wait... who... am I?\""); }
        }
        if state.elapsed_secs >= 24.0 {
            ui.label("");
            ui.label("He sits up, squinting through a pounding headache. Naked. Completely naked. He looks down at his situation and gives a short, pained laugh, immediately regretting it as a fresh wave of nausea threatens to hurl him back to the puddle. He notices a vague reflection in a broken mirror lying nearby—his face, a twisted shadow of former glory.");
        }

        if state.elapsed_secs >= 31.0 {
            ui.label("");
            if      state.elapsed_secs < 31.5 { ui.label("Memory flash."); }
            else if state.elapsed_secs < 32.5 { ui.label("Memory flash. Something went wrong."); }
            else if state.elapsed_secs < 34.5 { ui.label("Memory flash. Something went wrong. The Cult of Falk."); }
            else if state.elapsed_secs < 35.5 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination."); }
            else if state.elapsed_secs < 36.5 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign."); }
            else if state.elapsed_secs < 37.5 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny."); }
            else if state.elapsed_secs < 39.5 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone."); }
            else if state.elapsed_secs < 40.0 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now,"); }
            else if state.elapsed_secs < 40.3 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self,"); }
            else if state.elapsed_secs < 41.6 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self, weak and disoriented,"); }
            else if state.elapsed_secs < 42.0 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self, weak and disoriented, he's stuck in some alternate version of reality."); }
            else if state.elapsed_secs < 44.0 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self, weak and disoriented, he's stuck in some alternate version of reality. But there’s still one thing burning in the back of his mind:"); }
            else if state.elapsed_secs < 46.0 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self, weak and disoriented, he's stuck in some alternate version of reality. But there’s still one thing burning in the back of his mind: Conquer."); }
            else if state.elapsed_secs < 47.0 { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self, weak and disoriented, he's stuck in some alternate version of reality. But there’s still one thing burning in the back of his mind: Conquer. The."); }
            else                              { ui.label("Memory flash. Something went wrong. The Cult of Falk. World domination. His reign. His destiny. All gone. Now, a shadow of his former self, weak and disoriented, he's stuck in some alternate version of reality. But there’s still one thing burning in the back of his mind: Conquer. The. World."); }
        }

        if state.elapsed_secs >= 48.0 {
            ui.label("");
            ui.label("He stumbles to his feet, clutching the side of a filthy dumpster for support.");
        }

        if state.elapsed_secs >= 53.0 {
            ui.label("");
            if      state.elapsed_secs < 55.0 { ui.label("Falk: \"First things first.\"");                                                                            }
            else if state.elapsed_secs < 58.0 { ui.label("Falk: \"First things first. Clothes.\"");                                                                   }
            else if state.elapsed_secs < 63.0 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out\"");              }
            else if state.elapsed_secs < 65.3 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out.\"");             }
            else if state.elapsed_secs < 65.6 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out..\"");            }
            else if state.elapsed_secs < 65.9 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out...\"");           }
            else if state.elapsed_secs < 69.0 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out... unless\"");    }
            else if state.elapsed_secs < 69.3 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out... unless.\"");   }
            else if state.elapsed_secs < 69.6 { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out... unless..\"");  }
            else                              { ui.label("Falk: \"First things first. Clothes. Can’t exactly lead a global uprising with my ass out... unless...\""); }
        }

        if state.elapsed_secs >= 72.0 {
            ui.label("");
            ui.label("...");
        }

        if state.elapsed_secs >= 75.0 {
            ui.label("");
            if      state.elapsed_secs < 78.0 { ui.label("His eyes narrow, but then he shakes his head");                         }
            else if state.elapsed_secs < 80.0 { ui.label("His eyes narrow, but then he shakes his head—No,");                     }
            else                              { ui.label("His eyes narrow, but then he shakes his head—No, one step at a time."); }
        }

        if state.elapsed_secs >= 83.0 {
            ui.label("");
            if ui.button("Look Around").clicked() {}
        }
    });
}



impl eframe::App for IdleApp {
    fn save( &mut self, storage: &mut dyn eframe::Storage ) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update( &mut self, ctx: &egui::Context, _frame: &mut eframe::Frame ) {

        // egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        //     egui::menu::bar(ui, |ui| {
        //         if ui.button("Attributes").clicked() {}
        //         if ui.button("Inventory").clicked() {}
        //         if ui.button("Base").clicked() {}
        //         if ui.button("Research").clicked() {}
        //         if ui.button("Objectives").clicked() {}
        //         if ui.button("Explore").clicked() {}
        //     });
        // });

        if let State::Intro{..} = self.state {
            intro_update( self, ctx );
        }

        /*
        // Status bar:
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Attributes").clicked() {}
                if ui.button("Inventory").clicked() {}
                if ui.button("Base").clicked() {}
                if ui.button("Research").clicked() {}
                if ui.button("Objectives").clicked() {}
                if ui.button("Explore").clicked() {}

                //ui.menu_button("Player", |ui| {
                //    if ui.button("Attributes").clicked() {
                //        //functionality
                //    }
                //    if ui.button("Inventory").clicked() {
                //        std::process::exit(0);
                //    }
                //});

                //ui.menu_button("Base", |ui| {
                //    if ui.button("Cut").clicked() {
                //        //functionality
                //    }
                //    if ui.button("Copy").clicked() {
                //        //functionality
                //    }
                //    if ui.button("Paste").clicked() {
                //        //funtionality
                //    }
                //})
            });
        });

        // Left side Panel:
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
           ui.label("Hello World!");
        });

        // Right side Panel:
        egui::SidePanel::right("right_panel").show(ctx, |ui| {
           ui.label("Poopoo peepee!");
        });

        // Main screen:
        egui::CentralPanel::default().show( ctx, |ui| {
            ui.heading("Idle App");
            if ui.button("Start").clicked() {
                println!("Click!");
            }
            ui.label( format!("Name: {}", self.player.name) );
            ui.label(
                format!(
                    "HP: {}/{} | TUF: {} | END: {} | DEX: {} | INT: {} | DOM: {}",
                    self.player.life.cur,
                    self.player.life.max,
                    self.player.attributes.toughness,
                    self.player.attributes.endurance,
                    self.player.attributes.dexterity,
                    self.player.attributes.intellect,
                    self.player.attributes.dominance
                )
            );
        });*/

        // Update time:
        ctx.input( |i| self.elapsed_secs += self.time_factor * i.stable_dt );

        // Request repaint (this is to make time continuous instead of dependent on activity):
        ctx.request_repaint();
    }
}





















#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result {
    // env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([640.0,480.0])
            .with_min_inner_size([320.0,240.0])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon-256.png")[..]
                ).expect("Failed to load icon!"),
            ),
        ..Default::default()
    };
    eframe::run_native(
        "Idle App.",
        options,
        Box::new( |cc| {
            // egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok( Box::new( IdleApp::new(cc) ) )
        })
    )
}


#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local( async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(
                    |cc| Ok( Box::new( IdleApp::new(cc) ) )
                ),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p>The app has crashed. See the developer console for details.</p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

