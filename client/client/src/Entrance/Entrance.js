import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

import "./Entrance.css"
import 'bootstrap/dist/css/bootstrap.min.css';

import Registration from './Registration';
import Authorization from './Authorization';
import Recovery from './Recovery';

const EntranceState = {
    Authorization: 0,
    Registration: 1,
    Recovery: 2,
}

class Entrance extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            state: EntranceState.Authorization,
        }
    }
    
    render() {
        return(
            <div className='Entrance'>
                <div className='Form'>
                    {this.render_state()}
                </div>
            </div>
        );
    }

    render_state = () => {
        switch (this.state.state) {
            case EntranceState.Authorization:
                return (<Authorization move_to_registration={this.move_to_registration} move_to_recovery={this.move_to_recovery}/>);
            case EntranceState.Registration:
                return (<Registration move_to_authorization={this.move_to_authorization}/>);
            case EntranceState.Recovery:
                return (<Recovery move_to_authorization={this.move_to_authorization}/>);
        }
    }

    move_to_registration = () => {
        this.setState({
            state: EntranceState.Registration
        });
    }

    move_to_authorization = () => {
        this.setState({
            state: EntranceState.Authorization
        });
    }

    move_to_recovery = () => {
        this.setState({
            state: EntranceState.Recovery
        });
    }
}

export default Entrance;