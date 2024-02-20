import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

class Authorization extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            mail: '',
            password: '',

            authorization_status: {
                received: false,
                result: false,
            }
        }
    }

    render() {
        return(
            <div>
                {this.state.authorization_status.received ? <this.render_authorization_status/> : <this.render_form/>}
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
            </div>
        );
    }

    render_authorization_status = () => {
        return(
            <div>
                <p className='Title'>
                    {this.state.authorization_status.result ? 'Authorization succeed' : 'Authorization failed'}
                </p>

                <div className='Result'>
                    <Button variant="warning" type="submit" onClick={this.close_authorization_status}>Ok</Button>
                </div>
            </div>
        );
    }

    close_authorization_status = () => {
        this.setState({
            authorization_status: {
                received: false,
                result: false,
            }
        });
    }

    try_authorization = () => {
        let user_data = {
            mail: this.state.mail,
            password: this.state.password,
        };

        fetch('http://localhost:8080/hey', {
            method: 'GET',
            mode: 'no-cors',
        }).then(response => {
            this.setState({
                authorization_status: {
                    received: true,
                    result: true,
                }
            });
        }).catch(error => {
            console.log("something went wrong");
            this.setState({
                authorization_status: {
                    received: true,
                    result: false,
                }
            })
        });
    }
}

export default Authorization;