import React from 'react'
import Gameplay from "../Gameplay/Gameplay";
import Entrance from "../Entrance/Entrance";

const AppState = {
    Entrance: 0,
    Gameplay: 1,
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
            case AppState.Gameplay:
                return (<Gameplay/>)
        }
    }

    move_to_entrance = () => {
        this.setState({
            state: AppState.Entrance,
        });
    }

    move_to_game = (received_token) => {
        this.setState({
            state: AppState.Gameplay,
            token: received_token,
        });
    }
}

export default App;
