import { useState } from 'react';
import './entrance.css'
import Registration from './registration/registration';
import Authorization from './authorization/authorization';
import Recovery from './recovery/recovery';
import { ApplicationState } from '../application/application';


enum EntranceState {
    Authorization,
    Registration,
    Recovery,
}

type Props = {
    setApplicationState: (applicationState: ApplicationState) => void,
    setToken: (token: String) => void,
}


const Entrance: React.FC<Props> = (props) => {
    const [entranceState, setEntranceState] = useState<EntranceState>(EntranceState.Authorization);

    const RenderState = () => {
        switch (entranceState) {
            case EntranceState.Authorization:
                return <Authorization setEntranceState={setEntranceState} setApplicationState={props.setApplicationState} setToken={props.setToken}/>
            case EntranceState.Registration:
                return <Registration setEntranceState={setEntranceState}/>
            case EntranceState.Recovery:
                return <Recovery setEntranceState={setEntranceState}/>
        }
    }




    return(
        <div className='Entrance'>
            <div className='Form'>
                <RenderState/>
            </div>
        </div>
    )
}

export default Entrance;
export {EntranceState};