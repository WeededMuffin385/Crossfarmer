import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';
import './Game.css';
import Interface from './Interface/Interface'
import Garden from './Garden/Garden'

const GameState = {
    News: -1,
    Garden: 0,
    Inventory: 1,
    Auction: 2,
    Quests: 3,
}

class Game extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            state: GameState.Garden,
        }
    }

    render = () => {
        return(
            <div className='Game'>
                <Interface/>
                {this.render_state()}
            </div>
        );
    }

    render_state = () => {
        switch(this.state.state) {
            case GameState.News:
                return (<>There should be news</>);
            case GameState.Garden:
                return (<Garden/>);
        }
    }
}

export default Game;