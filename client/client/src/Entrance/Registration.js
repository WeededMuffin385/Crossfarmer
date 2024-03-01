import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

class Registration extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            mail: '',
            username: '',
            password: '',


            status: {
                received: false,
                result: false,
                error: '',
            },
        }
    }

    render = () => {
        return(
            <div>
                {this.state.status.received ? <this.render_status/> : <this.render_form/>}
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

    render_status = () => {
        return(
            <div>
                <p className='Title'>
                    {this.state.status.result ? 'Registration succeed' : 'Registration failed: ' + this.state.status.error}
                </p>

                <div className='Result'>
                    <Button variant="warning" type="submit" onClick={this.close_status}>Ok</Button>
                </div>
            </div>
        );
    }

    close_status = () => {
        this.setState({
            status: {
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
        }).then((response) => {
            if (response.ok) {
                this.setState({
                    status: {
                        received: true,
                        result: true,
                    }
                })
            } else {
                response.json().then((data)=>{
                    this.setState({
                        status: {
                            received: true,
                            result: false,
                            error: data.error,
                        }
                    })
                })
            }
        }).catch(error => {
            this.setState({
                status: {
                    received: true,
                    result: false,
                    error: "Connection lost.",
                }
            })
        });
    }
}

export default Registration;