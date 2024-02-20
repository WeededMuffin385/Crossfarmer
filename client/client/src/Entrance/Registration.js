import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

class Registration extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            mail: '',
            username: '',
            password: '',


            registation_status: {
                received: false,
                result: false,
            },
        }
    }

    render = () => {
        return(
            <div>
                {this.state.registation_status.received ? <this.render_registration_status/> : <this.render_form/>}
            </div>
        );
    }

    render_form = () => {
        return(
            <div>
                <p className='Title'>
                    Registration
                </p>

                <Form.Group>
                    <Form.Label>Email address</Form.Label>
                    <Form.Control value={this.state.mail} onChange={e => this.setState({mail: e.target.value})} type="email" size="lg" placeholder="Email"/>
                </Form.Group>
            
                <br/>

                <Form.Group>
                    <Form.Label>Username</Form.Label>
                    <Form.Control value={this.state.username} onChange={e => this.setState({username: e.target.value})} type="username" size="lg" placeholder="Username"/>
                </Form.Group>

                <br/>

                <Form.Group>
                    <Form.Label>Password</Form.Label>
                    <Form.Control value={this.state.password} onChange={e => this.setState({password: e.target.value})} type="password" size="lg" placeholder="Password"/>
                </Form.Group>

                <br/>

                <div className='button-container'>
                    <Button variant="primary" type="submit" onClick={this.try_registration}>Sign up</Button>
                    <Button variant="warning" type="submit" onClick={this.props.move_to_authorization}>Already have an account</Button>
                </div>
            </div>
        );
    }

    render_registration_status = () => {
        return(
            <div>
                <p className='Title'>
                    {this.state.registation_status.result ? 'Registration succeed' : 'Registration failed'}
                </p>

                <div className='Result'>
                    <Button variant="warning" type="submit" onClick={this.close_registration_status}>Ok</Button>
                </div>
            </div>
        );
    }

    close_registration_status = () => {
        this.setState({
            registation_status: {
                received: false,
                result: false,
            }
        });
    }

    try_registration = () => {
        let registration_data = {
            mail: this.state.mail,
            username: this.state.username,
            password: this.state.password,
        };

        let hostname = window.location.hostname;

        fetch('http://' + hostname + ':8080/registration', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify(registration_data),
        }).then(response => {
            if(!response.ok) throw new Error(response.status);

            this.setState({
                registation_status: {
                    received: true,
                    result: true,
                }
            });
        }).catch(error => {
            console.log("Something went wrong");
            this.setState({
                registation_status: {
                    received: true,
                    result: false,
                }
            })
        });
    }
}

export default Registration;