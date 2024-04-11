import React from 'react'
import './Creatures.css';


class Creatures extends React.Component {
    constructor(props) {
        super(props);


        this.setState({
            canvas: React.createRef()
        });
    }

    render = () => {
        return(
            <div className='Creatures'>
                
            </div>
        )
    }
}


export default Creatures;