import React from 'react'
import { Form, FormGroup, FormControl, Button } from 'react-bootstrap';

class Recovery extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
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
        )
    }
    
    render_form = () => {
        return(
            <div>
                <p className='Title'>
                    Recovery
                </p>

                <Form.Group>
                    <Form.Label>Email address</Form.Label>
                    <Form.Control value={this.state.mail} onChange={e => this.setState({mail: e.target.value})} type="email" size="lg" placeholder="Email"/>
                </Form.Group>

                <br/>

                <div className='button-container'>
                    <Button variant="primary" type="submit" onClick={this.try_recovery}>Recover password</Button>
                    <Button variant="warning" type="submit" onClick={this.props.move_to_authorization}>Already have an account</Button>
                </div>
            </div>
        );
    }

    render_status =  () => {
        return(
            <div>
                <p className='Title'>
                    {this.state.status.result ? 'Recovery mail sent' : 'Recovery failed: ' + this.state.status.error}
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

    try_recovery = () => {
        let hostname = window.location.hostname;

        let recovery_data = {
            mail: this.state.mail,
        };

        fetch('http://' + hostname + ':8080/recovery', {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify(recovery_data),
        }).then((response) => { 
            if (response.ok) {
                this.setState({
                    status: {
                        received: true,
                        result: true,
                    }
                })
            } else {
                response.json().then((data) => {
                    this.setState({
                        status: {
                            received: true,
                            result: false,
                            error: data.error
                        }
                    })
                });
            }
        }).catch(error => {
            this.setState({
                status: {
                    received: true,
                    result: false,
                    error: 'Connection lost.',
                }
            })
        });
    }
}

export default Recovery;