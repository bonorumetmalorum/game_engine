extern crate papi;
extern crate ecs;
extern crate gnuplot;

use ecs::*;
use ecs::entity::EntityIndex;
use ecs::component::Component;
use ecs::component::DenseComponentStorage;
use ecs::component::ComponentIterator;
use ecs::component::ComponentIteratorMut;
use ecs::component::Iter;
use std::fs::File;
use std::io::Write;
use std::hash::BuildHasherDefault;
use std::fs::create_dir;
use std::process::Command;
use gnuplot::Figure;
use gnuplot::Major;
use gnuplot::Color;
use gnuplot::BorderColor;
use gnuplot::Fix;
use gnuplot::AxesCommon;
use gnuplot::Auto;



const STANDARD: usize = 10000;

#[derive(Clone)]
struct R{
    pub x: f32
}

impl Component for R{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
struct W1{
    pub x: f32
}

impl Component for W1{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
        unimplemented!()
    }
}

#[derive(Clone)]
struct W2{
    pub x: f32
}

impl Component for W2{
    type ComponentStorage = DenseComponentStorage<Self>;

    fn update(&mut self) {
    }
}

fn build(number: usize) -> (Vec<EntityIndex>, ECS) {
    let mut entities = vec![];
    let mut ecs = ECS::new();
    for _ in 0..number {
        entities.push(ecs.allocate_new_entity());
    }
    (entities, ecs)
}

fn setup() -> ECS {
    let (entities, mut ecs) = build(STANDARD);
    ecs.register_new_component::<R>().expect("unable to register new component");
    ecs.register_new_component::<W1>().expect("unable to register new component");
    ecs.register_new_component::<W2>().expect("unable to register new component");
    for ent in entities {
        ecs.add_component(ent, R { x: 32.0 }).expect("not registered");
        ecs.add_component(ent, W1 { x: 0.0 }).expect("not registered");
        ecs.add_component(ent, W2 { x: 0.0 }).expect("not registered");
    }
    ecs
}

fn system_w1(read_r: ComponentIterator<R>, write_w1: ComponentIteratorMut<W1>) {
    let joint = read_r.join(write_w1);
    let iterator = joint.into_iterator_wrapper();
    for (r, w1) in iterator {
        w1.x = r.x;
    }
}

fn system_w2(read_r: ComponentIterator<R>, write_w2: ComponentIteratorMut<W2>){
    let joint = read_r.join(write_w2);
    let iterator = joint.into_iterator_wrapper();
    for (r, w2) in iterator {
        w2.x = r.x;
    }
}

fn main() {
    create_dir("./data/");

    {
        //setup 10000 empty
        let counters = &[papi::Counter::PAPI_L1_DCM, papi::Counter::PAPI_L2_DCM];
        let mut counters = unsafe {
            papi::CounterSet::new(counters)
        };
        let start = counters.read();
         build(STANDARD);
        let stop = counters.accum();

        println!("setup 10000 empty entities {} L1 misses, {} L2 misses",
                 stop[0] - start[0], stop[1] - start[1]);

        let result_arr = stop.iter().zip(start.iter()).map(|(x, y)| (x - y) as f32).collect::<Vec<f32>>();
        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "./data/setup_10000_empty_entities.png");

        fg.axes2d()
            .boxes(&[1., 2.], &result_arr, &[Color("gray"), BorderColor("black")])
            .set_title("setup 10000 empty entities", &[])
            .set_x_ticks_custom(
                vec![
                    Major(1. as f32, Fix("L1 Data Cache Miss".into())),
                    Major(2. as f32, Fix("L2 Data Cache Miss ".into())),
                ],
                &[],
                &[],
            )
            .set_y_range(Fix(0.0), Auto);
        fg.show();

    }

    {
        //setup 10000 with components
        let counters = &[papi::Counter::PAPI_L1_DCM, papi::Counter::PAPI_L2_DCM];
        let mut counters = unsafe {
            papi::CounterSet::new(counters)
        };
        let start = counters.read();
        let mut ecs = setup();
        let stop = counters.accum();

        println!("allocated 10000 entities with components with {} L1 misses, {} L2 misses",
                 stop[0] - start[0], stop[1] - start[1]);
        let data = format!("0 L1 {}\n1 L2 {}", stop[0] - start[0], stop[1] - start[1]);

        let result_arr = stop.iter().zip(start.iter()).map(|(x, y)| (x - y) as f32).collect::<Vec<f32>>();
        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "./data/setup_10000_entities_with_components.png");

        fg.axes2d()
            .boxes(&[1., 2.], &result_arr, &[Color("gray"), BorderColor("black")])
            .set_title("setup 10000 entities with components", &[])
            .set_x_ticks_custom(
                vec![
                    Major(1. as f32, Fix("L1 Data Cache Miss".into())),
                    Major(2. as f32, Fix("L2 Data Cache Miss ".into())),
                ],
                &[],
                &[],
            )
            .set_y_range(Fix(0.0), Auto);
        fg.show();
    }

    {
        //update
        let counters = &[papi::Counter::PAPI_L1_DCM, papi::Counter::PAPI_L2_DCM];
        let mut counters = unsafe {
            papi::CounterSet::new(counters)
        };
        let mut ecs = setup();
        let hr1 = ecs.get_component_read_handle::<R>();
        let hr2 = ecs.get_component_read_handle::<R>();
        let mut hw1 = ecs.get_component_write_handle::<W1>();
        let mut hw2 = ecs.get_component_write_handle::<W2>();
        let itrr1 = hr1.get_iterator();
        let itrr2 = hr2.get_iterator();
        let itrw1 = hw1.get_mut_iter();
        let itrw2 = hw2.get_mut_iter();

        let start = counters.read();
        system_w1(itrr1, itrw1);
        system_w2(itrr2, itrw2);
        let stop = counters.accum();

        println!("updated components sequentially with {} L1 misses, {} L2 misses",
                 stop[0] - start[0], stop[1] - start[1]);

        let result_arr = stop.iter().zip(start.iter()).map(|(x, y)| (x - y) as f32).collect::<Vec<f32>>();
        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "./data/update_10000_entities_with_components.png");
        fg.axes2d()
            .boxes(&[1., 2.], &result_arr, &[Color("gray"), BorderColor("black")])
            .set_title("update 10000 entities with components", &[])
            .set_x_ticks_custom(
                vec![
                    Major(1. as f32, Fix("L1 Data Cache Miss".into())),
                    Major(2. as f32, Fix("L2 Data Cache Miss ".into())),
                ],
                &[],
                &[],
            )
            .set_y_range(Fix(0.0), Auto);
        fg.show();
    }
}

