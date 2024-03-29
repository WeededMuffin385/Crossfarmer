import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

class Authorization extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            mail: '',
            password: '',

            status: {
                received: false,
                result: false,
                error: '',
            }
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
                    Authorization
                </p>

                <Form.Group>
                    <Form.Label>Email address</Form.Label>
                    <Form.Control value={this.state.mail} onChange={e => this.setState({mail: e.target.value})} type="email" size="lg" placeholder="Email"/>
                </Form.Group>
                
                <br/>

                <Form.Group>
                    <Form.Label>Password</Form.Label>
                    <Form.Control value={this.state.password} onChange={e => this.setState({password: e.target.value})} type="password" size="lg" placeholder="Password"/>
                </Form.Group>

                <br/>

                <div className='button-container'>
                    <Button variant="primary" type="submit" onClick={this.try_authorization}>Login</Button>
                    <Button variant="warning" type="submit" onClick={this.props.move_to_registration}>No account</Button>
                </div>

                <br/>

                <Button variant="danger" type="submit" onClick={this.props.move_to_recovery}>Forgot password</Button>

            </div>
        );
    }

    render_status = () => {
        return(
            <div>
                <p className='Title'>
                    {this.state.status.result ? 'Authorization succeed' : 'Authorization failed: ' + this.state.status.error}
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

    try_authorization = () => {
        let authorization_data = {
            mail: this.state.mail,
            password: this.state.password,
        };

        let hostname = window.location.hostname;

        fetch('http://' + hostname + ':8080/authorization', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(authorization_data),
        }).then((response) => {
            if (response.ok) {
                response.json().then((data) => {
                    this.props.move_to_game(data.token);
                });
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

export default Authorization;