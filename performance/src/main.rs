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
    pub x: f32,
    pub y: f32
}

impl Component for R{
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
struct W1{
    pub x: f32,
    pub y: f32
}

impl Component for W1{
    type ComponentStorage = DenseComponentStorage<Self>;
}

#[derive(Clone)]
struct W2{
    pub x: f32,
    pub y: f32
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
        ecs.add_component(ent, R { x: 32.0, y:10.0}).expect("not registered");
        ecs.add_component(ent, W1 { x: 10.0, y: 10.0 }).expect("not registered");
        ecs.add_component(ent, W2 { x: 10.0, y: 10.0}).expect("not registered");
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

        //ramp up tests - total sample size will be 10000
        const NUM_SAMPLE: usize = 10000;
        const NUM_STEP: usize = 1;
        let counters = &[papi::Counter::PAPI_REF_CYC];
        let mut counters = unsafe {
            papi::CounterSet::new(counters)
        };

        //setup all ds
        let mut ecs = setup(10000);

        //create test
        let mut createxaxis: Vec<usize> = Vec::new();
        let mut cpucreatecycles: Vec<i64> = Vec::new();
        let mut test_ecs = ECS::new();
        for i in 0..NUM_SAMPLE{
            let start = counters.read();
            test_ecs.allocate_new_entity();
            let stop = counters.accum();
            cpucreatecycles.push(stop[0] - start[0]);
            createxaxis.push(i);
        }

        //plotting
        for el in cpucreatecycles.iter().zip(createxaxis.iter()) {
            println!("cpu cycles vs x ticks {}, {}", el.0, el.1);
        }

        let mut fg = Figure::new();
        fg.set_terminal("pngcairo size 800, 600", "./data/cpucreatecycles.png");
        fg.axes2d()
            .set_title("CPU clock count vs create entities", &[])
            .set_legend(Graph(1.0), Graph(0.5), &[], &[])
            .lines(createxaxis.iter(), cpucreatecycles.iter(), &[Color("blue")])
            .set_x_ticks(Some((Auto, 0)), &[], &[])
            .set_y_ticks(Some((Auto, 0)), &[], &[])
            .set_x_label("number of entities", &[])
            .set_y_label("Clock cycles", &[]);
        fg.show();


        //read test
        println!("cpu cycle vs read test");
        let hr1 = ecs.get::<R>();
        let mut itrr1 = hr1.get_iter();
        let mut readxaxis: Vec<usize> = Vec::new();
        let mut cpureadcycles: Vec<i64> = Vec::new();
        //data collection
        for i in (0..NUM_SAMPLE).step_by(NUM_STEP){
            println!("testing for {} entities", i);
            //collection of data (need to get entities up to i)
            let start = counters.read();
            //read
            let var = itrr1.next_element(None);
            let stop = counters.accum();
            readxaxis.push(i);
            //store results
            cpureadcycles.push(stop[0] - start[0]);
        }
        println!("plotting");
        //plotting

        for el in cpureadcycles.iter().zip(readxaxis.iter()) {
            println!("cpu cycles vs x ticks {}, {}", el.0, el.1);
        }

        let mut fg = Figure::new();
        fg.set_terminal("pngcairo size 800, 600", "./data/cpureadcycles.png");
        fg.axes2d()
            .set_title("CPU clock count vs reading entities", &[])
            .set_legend(Graph(1.0), Graph(0.5), &[], &[])
            .lines(readxaxis.iter(), cpureadcycles.iter(), &[Color("blue")])
            .set_x_ticks(Some((Auto, 0)), &[], &[])
            .set_y_ticks(Some((Auto, 0)), &[], &[])
            .set_x_label("number of entities", &[])
            .set_y_label("Clock cycles", &[]);
        fg.show();

        //write test
        println!("cpu cycle vs write test");

        let mut writexaxis: Vec<usize> = Vec::new();
        let mut cpuwritecycles: Vec<i64> = Vec::new();
        let mut w1handle = ecs.get_mut::<W1>(); //write handle to data
        let mut w1iter = w1handle.get_mut_iter(); //write iter to data
        for i in (0..NUM_SAMPLE).step_by(NUM_STEP){
            println!("testing for {} entities", i);
            //collection of data (need to get entities up to i)
            let start = counters.read();
            //read
            let var = w1iter.next_element(None);
            let stop = counters.accum();
            writexaxis.push(i);
            //store results
            cpuwritecycles.push(stop[0] - start[0]);
        }
        println!("plotting");
        //plotting

        for el in cpuwritecycles.iter().zip(writexaxis.iter()) {
            println!("cpu cycles vs x ticks {}, {}", el.0, el.1);
        }

        let mut fg = Figure::new();
        fg.set_terminal("pngcairo size 800, 600", "./data/cpuwritecycles.png");
        fg.axes2d()
            .set_title("CPU clock count vs write entities", &[])
            .set_legend(Graph(1.0), Graph(0.5), &[], &[])
            .lines(writexaxis.iter(), cpuwritecycles.iter(), &[Color("blue")])
            .set_x_ticks(Some((Auto, 0)), &[], &[])
            .set_y_ticks(Some((Auto, 0)), &[], &[])
            .set_x_label("number of entities", &[])
            .set_y_label("Clock cycles", &[]);
        fg.show();
    }
}

