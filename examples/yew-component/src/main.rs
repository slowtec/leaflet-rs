use crate::components::control::{Cities, Control};
use crate::components::map_component::{City, MapComponent, Point};
use yew::prelude::*;
mod components;

enum Msg {
    SelectCity(City),
}

struct Model {
    city: City,
    cities: Cities,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let aachen = City {
            name: "Aachen".to_string(),
            lat: Point(50.7597f64, 6.0967f64),
        };
        let stuttgart = City {
            name: "Stuttgart".to_string(),
            lat: Point(48.7784f64, 9.1742f64),
        };
        let cities: Cities = Cities {
            list: vec![aachen, stuttgart],
        };
        let city = cities.list[0].clone();
        Self { city, cities }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectCity(city) => {
                self.city = self
                    .cities
                    .list
                    .iter()
                    .find(|c| c.name == city.name)
                    .unwrap()
                    .clone();
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let cb = ctx.link().callback(|name| Msg::SelectCity(name));
        html! {
            <>
                <MapComponent city={&self.city}  />
                <Control select_city={cb} cities={&self.cities}/>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
