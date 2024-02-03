use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Ennemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_position = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_position = *pos);

    let mut ennemies = <(Entity, &Point)>::query().filter(component::<Ennemy>());

    ennemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_position)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
