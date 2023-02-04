use gloo::timers::callback::Interval;
use yew::prelude::*;

enum Msg {
    RotX,
    RotY,
    RotZ,
    RotRev, // Added Feature
    VelocityIncrement, // Added Feature
    VelocityDecrement, // Added Feature
    AddBox, // Added Feature
    RemoveBox, // Added Feature
    Click,
}

#[allow(dead_code)]
struct Spawn {
    value: i64
}

struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

struct Point2D {
    x: f64,
    y: f64,
}

struct Edge(usize, usize);

struct Angle3D {
    xa: f64,
    ya: f64,
    za: f64,
}

type AngleVelocity = Angle3D;

struct Shape {
    shapes: Vec<Spawn>,
    vertices: Vec<Point3D>,
    edges: Vec<Edge>,
    angle_velocity: AngleVelocity,
    direction_multiplier: bool,
    velocity_multiplier: f64,
}

#[allow(dead_code)]
enum Axis {
    X,
    Y,
    Z,
}

const VIEW_BOX_SIZE: f64 = 600.0;
const VIEW_CENTER: Point2D = Point2D {
    x: VIEW_BOX_SIZE / 2.0,
    y: VIEW_BOX_SIZE / 2.0,
};
const FRAME_RATE: f64 = 200.0;
const ONE_DEGREE_IN_RADIAN: f64 = 0.01745329255;    // ONE_DEGREE_IN_RADIAN * 180 = PI; 3.141592
const TEN_DEGREE_IN_RADIAN: f64 = ONE_DEGREE_IN_RADIAN * 10.0;
const ACCELERATE_BY: f64 = ONE_DEGREE_IN_RADIAN * 50.0;
const DAMPEN_PERCENT: f64 = 1.0 - (0.9 / FRAME_RATE);

impl Component for Shape {
    type Message = Msg;
    type Properties = ();

    // points are added in the form of binary.
    // 100 == 0; -100 == 1
    // 000, 001, 010, 011, 100, 101, 110, 111
    fn create(_ctx: &Context<Self>) -> Self {
        let vertices = vec![
            Point3D {
                x: 100.0,
                y: 100.0,
                z: 100.0,
            },
            Point3D {
                x: -100.0,
                y: 100.0,
                z: 100.0,
            },
            Point3D {
                x: 100.0,
                y: -100.0,
                z: 100.0,
            },
            Point3D {
                x: -100.0,
                y: -100.0,
                z: 100.0,
            },
            Point3D {
                x: 100.0,
                y: 100.0,
                z: -100.0,
            },
            Point3D {
                x: -100.0,
                y: 100.0,
                z: -100.0,
            },
            Point3D {
                x: 100.0,
                y: -100.0,
                z: -100.0,
            },
            Point3D {
                x: -100.0,
                y: -100.0,
                z: -100.0,
            },
        ];
        
        // lines drawn
        let edges = vec![
            Edge(0, 1),
            Edge(0, 2),
            Edge(0, 4),
            Edge(1, 5),
            Edge(1, 3),
            Edge(2, 3),
            Edge(2, 6),
            Edge(4, 5),
            Edge(4, 6),
            Edge(3, 7),
            Edge(6, 7),
            Edge(5, 7),
        ];

        Self {
            vertices,
            edges,
            angle_velocity: AngleVelocity {
                xa: TEN_DEGREE_IN_RADIAN,
                ya: TEN_DEGREE_IN_RADIAN,
                za: TEN_DEGREE_IN_RADIAN,
            },
            direction_multiplier: true,
            velocity_multiplier: 1.0,
            shapes: Vec::new(),
        }
    }

    fn changed(&mut self, _ctx: &Context<Self>) -> bool {
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let link = ctx.link().clone();
            Interval::new((1000.0 / FRAME_RATE) as u32, move || {
                link.send_message(Msg::Click)
            })
            .forget();
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {

        match msg {
            Msg::VelocityIncrement => self.velocity_multiplier *= 2.0,
            Msg::VelocityDecrement => self.velocity_multiplier /= 2.0,

            Msg::RotRev => self.direction_multiplier = !self.direction_multiplier,

            Msg::RotX => if self.direction_multiplier {
                            self.angle_velocity.xa += ACCELERATE_BY * self.velocity_multiplier
                        } else {
                            self.angle_velocity.xa -= ACCELERATE_BY * self.velocity_multiplier
                        },
            Msg::RotY => if self.direction_multiplier {
                            self.angle_velocity.ya += ACCELERATE_BY * self.velocity_multiplier
                        } else {
                            self.angle_velocity.ya -= ACCELERATE_BY * self.velocity_multiplier
                        },
            Msg::RotZ => if self.direction_multiplier {
                            self.angle_velocity.za += ACCELERATE_BY * self.velocity_multiplier
                        } else {
                            self.angle_velocity.za -= ACCELERATE_BY * self.velocity_multiplier
                        },
            
            Msg::AddBox => {
                self.shapes.push(Spawn { value: 0 });
            },
            Msg::RemoveBox => {
                self.shapes.pop();
            },

            Msg::Click => {}
        }
        

        fn rotate(angle: &AngleVelocity, point: &Point3D) -> Point3D {
            let angle_per_frame = AngleVelocity {
                xa: angle.xa / FRAME_RATE,
                ya: angle.ya / FRAME_RATE,
                za: angle.za / FRAME_RATE,
            };

            let rot_x_res = rotate_in_plane(point.y, point.z, angle_per_frame.xa);
            let new_point = Point3D {
                x: point.x,
                y: rot_x_res.0,
                z: rot_x_res.1,
            };

            let rot_y_res = rotate_in_plane(new_point.x, new_point.z, angle_per_frame.ya);
            let new_point = Point3D {
                x: rot_y_res.0,
                y: new_point.y,
                z: rot_y_res.1,
            };

            let rot_z_res = rotate_in_plane(new_point.x, new_point.y, angle_per_frame.za);
            let new_point = Point3D {
                x: rot_z_res.0,
                y: rot_z_res.1,
                z: new_point.z,
            };

            new_point
        }

        fn rotate_in_plane(axis1: f64, axis2: f64, angle: f64) -> (f64, f64) {
            (
                (axis1 * angle.cos() - axis2 * angle.sin()),
                (axis2 * angle.cos() + axis1 * angle.sin()),
            )
        }

        self.vertices = self
            .vertices
            .iter()
            .map(|point| rotate(&self.angle_velocity, point))
            .collect::<Vec<Point3D>>();

        self.angle_velocity = AngleVelocity {
            xa: self.angle_velocity.xa * DAMPEN_PERCENT,
            ya: self.angle_velocity.ya * DAMPEN_PERCENT,
            za: self.angle_velocity.za * DAMPEN_PERCENT,
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        fn project(point3d: &Point3D) -> Point2D {
            Point2D {
                x: point3d.x + VIEW_CENTER.x,
                y: point3d.y + VIEW_CENTER.y,
            }
        }

        let point2dvec = self
            .vertices
            .iter()
            .map(|point3d| project(point3d))
            .collect::<Vec<Point2D>>();

        let points = point2dvec
            .iter()
            .enumerate()
            .map(|(index, point)| {
                html! (
                    <>
                    <text x={(point.x + 5.0).to_string()} y={(point.y + 5.0).to_string()}>{index}</text>
                    <circle cx={point.x.to_string()} cy={point.y.to_string()} r="2" />
                    </>
                )
            })
            .collect::<Html>();

        let link = ctx.link();
        
            html! {
                <>
                    <div>
                    <h1>{ "CUBE ROTATION" }</h1>
                    <h3> {"PiXiCreate's feature implementation"} </h3>
                    </div>
                    <div>
                        <button onclick={link.callback(|_| Msg::RotX)}>{ "Rotate X" }</button>
                        <button onclick={link.callback(|_| Msg::RotY)}>{ "Rotate Y" }</button>
                        <button onclick={link.callback(|_| Msg::RotZ)}>{ "Rotate Z" }</button>
                        <br/>
                        <br/>
                        <button onclick={link.callback(|_| Msg::RotRev)}>{ "Reverse the Directions.!" }</button>
                        <br/>
                        <button onclick={link.callback(|_| Msg::VelocityIncrement)}>{ "Velocity ↑↑" }</button>
                        <button onclick={link.callback(|_| Msg::VelocityDecrement)}>{ "Velocity ↓↓" }</button>
                        <br/>
                        <br/>
                        <button onclick={link.callback(|_| Msg::AddBox)}>{ "Add a Cube +" }</button>
                        <button onclick={link.callback(|_| Msg::RemoveBox)}>{ "Remove a Cube -" }</button>
                
                        <svg viewBox="0.0 0.0 1000.0 475.0">
                            <g>
                                {
                                    self.edges
                                    .iter()
                                    .map(|edge| {
                                        let point1 = &point2dvec[edge.0];
                                        let point2 = &point2dvec[edge.1];
                                        html! {
                                        <line x1={point1.x.to_string()} y1={point1.y.to_string()} x2={point2.x.to_string()} y2={point2.y.to_string()} stroke="black"/>
                                        }
                                    })
                                    .collect::<Vec<Html>>()
                                }
                                {points}
                            </g>
                        </svg>
                        {
                            self.shapes.iter().map(|_spawn| {
                                html! {
                                    <div>
                                        <svg viewBox="0.0 0.0 1000.0 475.0">
                                            <g> {
                                                    self.edges
                                                    .iter()
                                                    .map(|edge| {
                                                        let point1 = &point2dvec[edge.0];
                                                        let point2 = &point2dvec[edge.1];
                                                        html! {
                                                            <line x1={point1.x.to_string()} y1={point1.y.to_string()} x2={point2.x.to_string()} y2={point2.y.to_string()} stroke="black">
                                                            </line>
                                                        }
                                                    })
                                                    .collect::<Vec<Html>>()
                                            } {
                                                point2dvec
                                                .iter()
                                                .enumerate()
                                                .map(|(index, point)| {
                                                    html! (
                                                        <>
                                                        <text x={(point.x + 5.0).to_string()} y={(point.y + 5.0).to_string()}>{index}</text>
                                                        <circle cx={point.x.to_string()} cy={point.y.to_string()} r="2" />
                                                        </>
                                                    )
                                                })
                                                .collect::<Html>()
                                            }
                                            </g>
                                        </svg>         
                                    </div>
                                }
                            }).collect::<Vec<Html>>()     
                        }
                    </div>
                </>
            }
    }
}

fn main() {
    yew::start_app::<Shape>();
}
