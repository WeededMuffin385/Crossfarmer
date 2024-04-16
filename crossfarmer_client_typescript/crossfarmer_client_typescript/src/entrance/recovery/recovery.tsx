import {EntranceState} from '../entrance'


type Props = {
    setEntranceState: (next_state: EntranceState) => void,
}






const Recovery: React.FC<Props> = (props) => {
    return (
        <div>
            <h1>
                Recovery
            </h1>

            <label htmlFor='mail'>Email address</label>
            <input id='mail' type='text' />

            <br/>

            <div className='ButtonContainer'>
                <button className='orange' onClick={()=>{}}>Recover password</button>
                <button className='green' type="submit" onClick={() => {props.setEntranceState(EntranceState.Authorization)}}>Already know password</button>
            </div>
        </div>
    )
}

export default Recovery;