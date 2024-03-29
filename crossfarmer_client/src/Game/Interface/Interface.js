import React from 'react'

import { useState } from 'react';
import Button from 'react-bootstrap/Button';
import Collapse from 'react-bootstrap/Collapse';


import './Interface.css';


import gem from './gemRed.png';
import coin from './coin.png';
import experience from './tome.png';

class Interface extends React.Component {
    constructor(props) {
        super(props);

        this.state = {
            open: false,
        }
    }

    render = () => {
        return(
            <div className='Interface'>
                <div className='Top'>
                    <div className='Object'>
                        <img src={coin} title="Ваш комментарий при наведении"/>
                    </div>
                    <div className='Object'>
                        <img src={gem} title="Ваш комментарий при наведении"/>
                    </div>
                    <div className='Object'>
                        <img src={experience} title="Ваш комментарий при наведении"/>
                    </div>

                    <div className='Object Button' onClick={() => this.setState({open: !this.state.open})}>
                        Menu
                    </div>
                </div>

                <Collapse in={this.state.open}>
                    <div className='Collapse' id="example-collapse-text">
                        <div className='Button'>
                            Garden
                        </div>
                        <div className='Button'>
                            Inventory
                        </div>
                        <div className='Button'>
                            Auction
                        </div>
                        <div className='Button'>
                            Quests
                        </div>
                    </div>
                </Collapse>
            </div>
        );
    }
}

export default Interface;