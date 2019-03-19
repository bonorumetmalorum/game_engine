extern crate papi;
extern crate ecs;
extern crate gnuplot;

use ecs::*;
use ecs::entity::EntityIndex;
use ecs::component::Component;
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
use gnuplot::Graph;
use gnuplot::AxesCommon;
use gnuplot::Auto;
use gnuplot::Caption;
use ecs::component::dense_component_storage::DenseComponentStorage;
use ecs::component::dense_component_storage::DenseComponentIterator;
use ecs::component::dense_component_storage::DenseComponentIteratorMut;
use ecs::component::iter::Iter;
use ecs::component::storage::Storage;
use gnuplot::Mirror;


const STANDARD: usize = 10000;

#[derive(Clone)]
struct R{
    pub x: f32
}

impl Component for R{
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
struct W1{
    pub x: f32
}

impl Component for W1{
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
struct W2{
    pub x: f32
}

impl Component for W2{
    type ComponentStorage = DenseComponentStorage<Self>;
}

fn build(number: usize) -> (Vec<EntityIndex>, ECS) {
    let mut entities = vec![];
    let mut ecs = ECS::new();
    for _ in 0..number {
        entities.push(ecs.allocate_new_entity());
    }
    (entities, ecs)
}

fn setup(sample_size: usize) -> ECS {
    let (entities, mut ecs) = build(sample_size);
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



fn system_w1(read_r: DenseComponentIterator<R>, write_w1: DenseComponentIteratorMut<W1>) {
    let joint = read_r.join(write_w1);
    let iterator = joint.into_iterator_wrapper();
    for (r, w1) in iterator {
        w1.x = r.x;
    }
}

fn system_w2(read_r: DenseComponentIterator<R>, write_w2: DenseComponentIteratorMut<W2>){
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
        let mut ecs = setup(STANDARD);
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
        let mut ecs = setup(STANDARD);
        let hr1 = ecs.get::<R>();
        let hr2 = ecs.get::<R>();
        let mut hw1 = ecs.get_mut::<W1>();
        let mut hw2 = ecs.get_mut::<W2>();
        let itrr1 = hr1.get_iter();
        let itrr2 = hr2.get_iter();
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

    {
        //ramp up test - total sample size will be 10000
        const NUM_SAMPLE: usize = 10000;
        const NUM_STEP: usize = 1000;
        let mut l1cachemiss: Vec<i64> = Vec::new();
        let mut l2cachemiss: Vec<i64> = Vec::new();
        let mut xaxis: Vec<usize> = Vec::new();
        let counters = &[papi::Counter::PAPI_L1_DCM, papi::Counter::PAPI_L2_DCM];
        let mut counters = unsafe {
            papi::CounterSet::new(counters)
        };

        for i in (0..NUM_SAMPLE).step_by(NUM_STEP){
            //setup
            let mut ecs = setup(i);
            let hr1 = ecs.get::<R>();
            let hr2 = ecs.get::<R>();
            let mut hw1 = ecs.get_mut::<W1>();
            let mut hw2 = ecs.get_mut::<W2>();
            let itrr1 = hr1.get_iter();
            let itrr2 = hr2.get_iter();
            let itrw1 = hw1.get_mut_iter();
            let itrw2 = hw2.get_mut_iter();
            //collection of data
            let start = counters.read();
            system_w1(itrr1, itrw1);
            system_w2(itrr2, itrw2);
            let stop = counters.accum();
            xaxis.push(i);
            //store results
            l1cachemiss.push(stop[0] - start[0]);
            l2cachemiss.push(stop[1] - start[1]);
        }
        //plotting

        for el in l2cachemiss.iter().zip(xaxis.iter()) {
            println!("{}, {}", el.0, el.1);
        }
        let mut fg = Figure::new();
        fg.set_terminal("pngcairo", "./data/l1andl2cachemissscalability.png");
        fg.axes2d()
            .set_title("L1 and L2 cache miss scalability", &[])
            .set_legend(Graph(1.0), Graph(0.5), &[], &[])
            .lines(xaxis.iter(), l1cachemiss.iter(), &[Caption("L1 cache misses"), Color("blue")])
            .lines(xaxis.iter(), l2cachemiss.iter(), &[Caption("L2 cache misses"), Color("red")])
            .set_x_ticks(Some((Fix(1000.0), 1)), &[], &[])
            .set_y_ticks(Some((Auto, 1)), &[], &[]);
        fg.show();
    }
}

