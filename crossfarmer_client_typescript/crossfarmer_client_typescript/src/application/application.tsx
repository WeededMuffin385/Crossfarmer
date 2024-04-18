import React, { useState } from 'react';
import './application.css'
import './button.css'
import Entrance from '../entrance/entrance';
import Gameplay from '../gameplay/gameplay';

enum ApplicationState {
    Gameplay,
    Entrance,
}

const Application = () => {
    const [applicationState, setApplicationState] = useState<ApplicationState>(ApplicationState.Entrance);
    const [token, setToken] = useState<string>('');

    const RenderState = () => {
        switch (applicationState) {
            case ApplicationState.Gameplay:
                return <Gameplay setApplicationState={setApplicationState} token={token}/>
            case ApplicationState.Entrance:
                return <Entrance setApplicationState={setApplicationState} setToken={setToken}/>
        }
    }

    return (
        <div className='Application'>
            <RenderState/>
        </div>
    )
}

export default Application;
export {ApplicationState};