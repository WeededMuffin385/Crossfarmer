import React from 'react'
import Game from "../Game/Game";
import Entrance from "../Entrance/Entrance";
import './App.css';

const AppState = {
    Entrance: 0,
    Game: 1,
}


class App extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            state: AppState.Entrance,
        };
    }

    render = () => {
        return this.render_state();
    }

    render_state = () => {
        switch (this.state.state) {
            case AppState.Entrance:
                return (<Entrance move_to_game={this.move_to_game} />);
            case AppState.Game:
                return (<Game/>)
        }
    }

    move_to_entrance = () => {
        this.setState({
            state: AppState.Entrance,
        });
    }

    move_to_game = (received_token) => {
        this.setState({
            state: AppState.Game,
            token: received_token,
        });
    }
}

export default App;
