import React from 'react'

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
                <h1>
                    Recovery
                </h1>

                <label for='mail'>Email address</label>
                <input id='mail' type='text' />

                <br/>

                <div className='button-container'>
                    <button className='orange' onClick={this.try_recovery}>Recover password</button>
                    <button className='green' type="submit" onClick={this.props.move_to_authorization}>Already know password</button>
                </div>
            </div>
        );
    }

    render_status =  () => {
        return(
            <div>
                <h1>
                    {this.state.status.result ? 'Recovery mail sent' : 'Recovery failed: ' + this.state.status.error}
                </h1>

                <div className='Result'>
                    <button onClick={this.close_status}>Ok</button>
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
            mail: document.getElementById("mail").value,
        };

        fetch('http://' + hostname + ':8080/api/entrance/recovery', {
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