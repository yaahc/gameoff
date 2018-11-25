use amethyst::{ecs::prelude::*, prelude::*};

pub struct Game<'a, 'b> {
    pub dispatcher: Dispatcher<'a, 'b>,
}

impl<'a, 'b> State<GameData<'a, 'b>, StateEvent> for Game<'a, 'b> {
    fn update(&mut self, data: StateData<GameData<'a, 'b>>) -> Trans<GameData<'a, 'b>, StateEvent> {
        data.data.update(&data.world);
        self.dispatcher.dispatch(&data.world.res);

        Trans::None
    }
}
