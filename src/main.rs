use std::time::Instant;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};

use md_wgpu::{Color, Lines, Point2, Renderer, Shape, Vector2};

mod widget;
use widget::{TargetPinpoint, Test, Widget};

mod stretch;

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    wgpu_subscriber::initialize_default_subscriber(None);
    let mut renderer = pollster::block_on(Renderer::new(&window));
    stretch::stretch(1600.0, 900.0);

    /*let mut polygon = Polygon::new(
        &mut renderer,
        Size {
            width: 0.7,
            height: 0.7,
        },
        10,
    );
    polygon.draw(&mut renderer);

    /*let mut vertices = [Vertex::default(); 4];
    vertices[0].position.y = -1.0;
    vertices[1].position.x = 1.0;
    vertices[2].position.y = 1.0;
    vertices[3].position.x = -1.0;
    let indices = [0, 1, 1, 2, 2, 3, 3, 0];
    let handle = renderer.lines_buffer.alloc(4, 8).unwrap();
    if let Some((v_mut, i_mut)) = renderer.lines_buffer.get_mut_slice(handle) {
        v_mut.copy_from_slice(&vertices);
        i_mut.copy_from_slice(&indices);
    }*/*/

    let mut lines = Lines::new(&mut renderer, Vector2::new(1.0, 1.0), 4);
    //lines.set_line_position(0, Point { x: 0.0, y: -1.0 }, Point { x: 1.0, y: 0.0 });
    lines.set_line_position(0, Point2::new(0.5, 0.0), Point2::new(1.0, 0.5));
    lines.set_line_color(0, Color::RED);
    //lines.set_line_position(1, Point { x: 1.0, y: 0.0 }, Point { x: 0.0, y: 1.0 });
    lines.set_line_position(1, Point2::new(1.0, 0.5), Point2::new(0.5, 1.0));
    lines.set_line_color(1, Color::GREEN);
    //lines.set_line_position(2, Point { x: 0.0, y: 1.0 }, Point { x: -1.0, y: 0.0 });
    lines.set_line_position(2, Point2::new(0.5, 1.0), Point2::new(0.0, 0.5));
    lines.set_line_color(2, Color::BLUE);
    //lines.set_line_position(3, Point { x: -1.0, y: 0.0 }, Point { x: 0.0, y: -1.0 });
    lines.set_line_position(3, Point2::new(0.0, 0.5), Point2::new(0.5, 0.0));
    lines.set_line_color(3, Color::GREEN);
    //lines.set_position(Point { x: 0.3, y: 0.3 });
    lines.draw(&mut renderer, Point2::new(0.0, 0.0));

    /*let mut rectangle = Rectangle::new(
        &mut renderer,
        Size {
            width: 0.3,
            height: 0.1,
        },
    );*/
    //rectangle.set_position(Point { x: -1.0, y: 0.0 });
    //rectangle.set_position(Point { x: 0.5, y: 0.5 });
    //rectangle.draw(&mut renderer);

    let mut test = Test::new(&mut renderer, Vector2::new(0.5, 0.5));
    test.set_position(Point2::new(0.5, 0.5));
    test.draw(&mut renderer);
    let mut tp = TargetPinpoint::new(&mut renderer, Vector2::new(0.5, 0.5));
    tp.draw(&mut renderer);

    let mut timing = Instant::now();
    let timing2 = Instant::now();
    let mut frames: u32 = 0;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !renderer.input(event) {
                    match event {
                        WindowEvent::CloseRequested => {
                            *control_flow = ControlFlow::Exit;
                            let elapsed = timing2.elapsed();
                            println!("Rendered {} frames in {:?}", frames, elapsed);
                        }
                        WindowEvent::Resized(physical_size) => {
                            /*let (stretch, nodes) = stretch::stretch(
                                physical_size.width as f32,
                                physical_size.height as f32,
                            )
                            .unwrap();*/
                            /*let phys_to_rel = Size {
                                width: 1.0,
                                height: 1.0,
                            } / Size::from(*physical_size);

                            let mul = Size {
                                width: 1.0,
                                height: 1.0,
                            } / Size::from(*physical_size);

                            let layout = stretch
                                .layout(*nodes.get("target_pinpoint").unwrap())
                                .unwrap();
                            let size = phys_to_rel * layout.size.into();
                            polygon.resize(size);
                            let mut pos = Point::from(mul) * Point::from(layout.location);
                            //pos.x -= 1.0; // Shift from centre to left side.
                            //pos.y = 1.0 - pos.y; // Shift from centre to top with flip.
                            println!("poly {:?}", size);
                            println!("poly {:?}", pos);
                            polygon.set_position(pos);
                            polygon.draw(&mut renderer);

                            let layout = stretch.layout(*nodes.get("block_map").unwrap()).unwrap();
                            rectangle.resize(mul * layout.size.into());
                            //rectangle.set_position(Point::from(mul) * Point::from(layout.location));
                            rectangle.draw(&mut renderer);

                            println!("resized {:?}", physical_size);
                            /*let aspect_ratio =
                                physical_size.width as f32 / physical_size.height as f32;
                            let scale = if aspect_ratio > 1.0 {
                                Size {
                                    width: 1.0 / aspect_ratio,
                                    height: 1.0,
                                }
                            } else {
                                Size {
                                    width: 1.0,
                                    height: aspect_ratio,
                                }
                            };
                            //polygon.scale_aspect(aspect_ratio);
                            polygon.scale(scale);
                            rectangle.scale(scale);
                            polygon.draw(&mut renderer);
                            rectangle.draw(&mut renderer);*/*/
                            renderer.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so we have to dereference it twice
                            println!("scalefactorchanged");
                            renderer.resize(**new_inner_size);
                        }
                        WindowEvent::KeyboardInput { input, .. } => match input {
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            } => *control_flow = ControlFlow::Exit,
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            Event::MainEventsCleared => {
                // Triggers every loop.
                //let elapsed = timing.elapsed();
                //println!("elapsed {:?}", elapsed);
                /*frames += 1;
                //timing = Instant::now();
                renderer.render_start();
                renderer.draw_text("test1", Point2::new(50.0, 50.0), Color::RED, 32.0);
                renderer.draw_text("test2", Point2::new(50.0, 70.0), Color::GREEN, 20.0);
                renderer.render_finish();*/

                //window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                println!("redraw req");
                let start = Instant::now();
                renderer.update();
                let elapsed = start.elapsed();
                println!("renderer update {:?}", elapsed);
                let start = Instant::now();
                /*match renderer.render() {
                    Ok(_) => {}
                    // Recreate the swap_chain if lost
                    Err(wgpu::SwapChainError::Lost) => renderer.resize(renderer.size()),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SwapChainError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }*/
                renderer.render_start();
                renderer.draw_text("test1", Point2::new(50.0, 50.0), Color::RED, 32.0);
                renderer.draw_text("test2", Point2::new(50.0, 70.0), Color::GREEN, 20.0);
                renderer.render_finish();
                let elapsed = start.elapsed();
                println!("renderer render {:?}", elapsed);
            }
            _ => {}
        }
    });
}
