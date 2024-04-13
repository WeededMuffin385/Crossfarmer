import React from 'react'
import './Gameplay.css';
import Interface from './Interface/Interface'
import Creatures from './Creatures/Creatures'

const GameState = {
    News: -1,
    Creatures: 0,
    Inventory: 1,
    Auction: 2,
    Quests: 3,
}

class Gameplay extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            state: GameState.News,
        }
    }

    render = () => {
        return(
            <div className='Gameplay'>
                <Interface change_state={this.change_state} GameState={GameState}/>
                <div className='Content'>
                    {this.render_state()}
                </div>
            </div>
        );
    }

    render_state = () => {
        switch(this.state.state) {
            case GameState.News:
                return (<>There should be newstyfghioasoi diosah iodiaos hiodh aioshdio ahsido hashd aihd sioashd ioahd ioa diso hdah dioah d</>);
            case GameState.Creatures:
                return (<Creatures/>);
        }
    }

    change_state = (state) => {
        this.setState({
            state: state,
        });
    }
}

export default Gameplay;