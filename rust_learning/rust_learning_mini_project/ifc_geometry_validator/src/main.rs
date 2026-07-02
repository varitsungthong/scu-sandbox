// enum geometry {
//     Empty, //hold none
//     Mesh(u32), //hold number of vertices in u32
//     Extrusion{area : f32, length : f32}
// }

// impl geometry {
//     fn get_volume (&mut self , area :f32, length : f32  ) {
//         self.area = area;
//         self.length = length;
//         let volume = area * length ;
//     }
//     fn quit(&mut self){
//         self.quit = true ;
//     }
//     fn Mesh(&mut self){
//         self.Mesh = true ;
//     }
//     fn process(&mut self , Geometry: geometry){
//             match {
//                 geometry::Extrusion(area,length) => self.get_volume(volume),
//                 geometry::Empty => self.quit(),
//                 geometry::Mesh(u32) => self.Mesh(),
//             }
//         }
// }



// fn main() {
//     let geometry1 = {Empty, Mesh(5), Extrusion{area : 5.0 ,  length : 5.0}}
//     let geometry2 = {Empty, Mesh(6), Extrusion{area : 6.0 ,  length : 7.0}}
//     let geometry3 = {Empty, Mesh(7), Extrusion{area : 9.0 ,  length : 9.0}}
//     let stack_geometry = [
//         geometry1, geometry2 , geometry3
//     ]
//     println!("Hello, world!");
// }
// (x: Option<i32>) -> Option<i32>

enum Instance {Empty , Mesh(u32) , Extrusion{ length :u32 , area :u32  }}
    

impl Instance {
    fn get_volume ( &Self) -> Option<u32>{
    //let volume = length * area;
    match self {
        Instance::Empty => None ,
        Instance::Mesh(u32) => Some(*u32) ,
        Instance::Extrusion{ length  , area   } => Some(length*area) ,
        }
    }
}

fn main() {
    let stack_array = [Instance::Mesh(67), Instance::Empty, Instance::Extrusion{length:69, area : 67  }];
    for instance in stack_array.iter() {
        if let Some(total_weight) = instance.get_volume() {
            println!("total weight is {}", total_weight)
        }
        else {
            println!("Unpriceable")
        }

    // let column = Instance::Extrusion {length :67,area :69};
    // let door = Instance::Mesh(69);
    // let sculpter = Instance::Empty ;

    // let column_volume = get_volume(column);
    // let door_quantity = get_volume(door);
    // if let get_volume(sculpture) = None {
    //     println!{"Unprieable"}
    // };
     

    // let column_price_per_m3 = 6969;
    // let column_price = column_volume * column_price_per_m3;
    // let door_price_per_unit = 6767;
    // let door_price = door_price_per_unit * door_quantity;

    // println!("The volume of column is {}", column_volume)
    // println!("The price of column is {} ", column_price)

    }
}