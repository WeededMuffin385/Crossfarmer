import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

import "./Entrance.css"
import 'bootstrap/dist/css/bootstrap.min.css';

import Registration from './Registration';
import Authorization from './Authorization';

const StateOfRequest = {
    Success: 'Success',
    Error: 'Error',
}

class Entrance extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            registered: true,
            authorized: false,

            mail: '',
            username: '',
            password: '',
        }
    }
    
    render() {
        return(
            <div className='Entrance'>
                <div className='Form'>
                    {this.state.registered ? <Authorization move_to_registration={this.move_to_registration}/> : <Registration move_to_authorization={this.move_to_authorization}/>}
                </div>
            </div>
        );
    }

    move_to_registration = () => {
        this.setState({
            registered: false,
        });
    }

    move_to_authorization = () => {
        this.setState({
            registered: true,
        });
    }
}

export default Entrance;