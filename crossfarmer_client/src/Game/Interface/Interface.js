import React from 'react'

import { useState } from 'react';

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
                    <div className='Indicator'>
                        <img src={coin} title="Ваш комментарий при наведении"/>
                    </div>
                    <div className='Indicator'>
                        <img src={gem} title="Ваш комментарий при наведении"/>
                    </div>
                    <div className='Indicator'>
                        <img src={experience} title="Ваш комментарий при наведении"/>
                    </div>

                    <button onClick={this.change_menu_visibility}>
                        Menu
                    </button>
                </div>

                <div className='Menu' id='menu'>
                    <button onClick={()=>{this.props.change_state(this.props.GameState.News)}}>News</button>
                    <button onClick={()=>{this.props.change_state(this.props.GameState.Creatures)}}>Creatures</button>
                    <button onClick={()=>{this.props.change_state(this.props.GameState.Inventory)}}>Inventory</button>
                    <button onClick={()=>{this.props.change_state(this.props.GameState.Auction)}}>Auction</button>
                    <button onClick={()=>{this.props.change_state(this.props.GameState.Quests)}}>Quests</button>
                </div>
            </div>
        );
    }

    change_menu_visibility = () => {
        if (document.getElementById('menu').style.display === 'none') {
            document.getElementById('menu').style.display = 'block';
        } else {
            document.getElementById('menu').style.display = 'none';
        }
    }
}

export default Interface;