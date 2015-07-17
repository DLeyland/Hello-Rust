struct Player_object{
	x_pos: i32,
	y_pos: i32,
	z_pos: i32,
	orientation: i16, //Oentation done in degrees, e.g 0-360
	hp: i8, //from 0 to 100
	class: i8,
	model_num: i8,
}

fn player_create(){
	let new_player = Player_object{0,0,0,0,100,3,1};
}

