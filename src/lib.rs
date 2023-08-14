pub mod game_of_life
{
    use std::collections::HashSet;

    type IType = i32;

    pub struct GameModel
    {
        alive_cells : HashSet<(IType, IType)>
    }

    impl GameModel {

        pub fn new() -> Self
        {
            GameModel { alive_cells : HashSet::new() }
        }

        pub fn new_game(&mut self)
        {
            self.alive_cells.clear()
        }
        
        // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
        // Any live cell with two or three live neighbours lives on to the next generation.
        // Any live cell with more than three live neighbours dies, as if by overpopulation.
        // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
        pub fn step(&mut self)
        {

            let mut new_alive_cells = HashSet::new();

            //we check every live cell
            for (x , y) in &self.alive_cells
            {
                //we check their neighbours 
                let mut count = 0;
                for i in [-1, 0, 1]
                {
                    for j in [-1 , 0, 1]
                    {
                        if i != j
                        {
                            //if the neighbour if live we count it
                            if self.alive_cells.contains( &(x + i , y + j) )
                            {
                                count += 1;
                            }
                            else { //if not we check its neighbours
                                let mut count_2 = 0;
                                for i_2 in [-1, 0, 1]
                                {
                                    for j_2 in [-1 , 0, 1]
                                    {
                                        if i_2 != j_2
                                        {
                                            if self.alive_cells.contains( &(x + i + i_2 , y + j + j_2) )
                                            {
                                                count_2 += 1;
                                            }
                                        }
                                    }
                                }
                                if count_2 == 3
                                {
                                    new_alive_cells.insert( (x + i, y + j) );
                                }
                            }
                        }
                    }
                }

                if [2, 3].contains(&count)
                {
                    new_alive_cells.insert( (*x , *y) );
                }
            }
        }
        
        
        pub fn get_alive_cells(&self) -> &HashSet<(IType, IType)>
        {
            &self.alive_cells
        }
    
        
        pub fn set_cell(&mut self, (x, y): (IType, IType), alive : bool)
        {
            //set cell to alive == add it to alive cells
            if alive {
                self.alive_cells.insert((x , y));
            } else {
                self.alive_cells.remove(&(x , y));
            }
        }

    }


}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game_of_life::*;
    
    
    #[test]
    fn set_cell_alive() {
        let mut model = GameModel::new();

        model.set_cell((0, 0), true);
        
        let mut my_set = HashSet::new();
        my_set.insert((0,0));

        assert_eq!(model.get_alive_cells().len(), my_set.len());
        assert_eq!(*model.get_alive_cells(), my_set);
    }

    #[test]
    fn set_cell_dead() {
        let mut model = GameModel::new();

        model.set_cell((0, 0), false);

        let my_set = HashSet::new();

        assert_eq!(model.get_alive_cells().len(), my_set.len());
        assert_eq!(*model.get_alive_cells(), my_set);
    }
}
