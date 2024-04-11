import React from 'react'

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
                <h1>
                    Registration
                </h1>


                <label for='mail'>Email address</label>
                <input id='mail' type='text' />

                <br/>
            
                <label for='username'>Username</label>
                <input id='username' type='text' />
                
                <br/>

                <label for='password'>Password</label>
                <input id='password' type='password' />

                <br/>

                

                <div className='button-container'>
                    <button className='green' onClick={this.try_registration}>Sign up</button>
                    <button className='orange' onClick={this.props.move_to_authorization}>Already have an account</button>
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
                    <button variant="warning" type="submit" onClick={this.close_status}>Ok</button>
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

        fetch('http://' + hostname + ':8080/api/entrance/registration', {
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