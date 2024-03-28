import React from 'react'
import './Garden.css';


class Garden extends React.Component {
    constructor(props) {
        super(props);


        this.setState({
            canvas: React.createRef()
        });
    }

    render = () => {
        return(
            <div className='Garden'>
                {this.render_canvas()}
            </div>
        )
    }


    render_canvas = () => {
        const canvas = this.state.canvas.current;
        const context = canvas.getContext('2d');

        const triangleData = {
            x1: 100,
            y1: 100,
            x2: 200,
            y2: 100,
            x3: 150,
            y3: 200,
          };
    
        // Получаем данные треугольника из JSON
        const { x1, y1, x2, y2, x3, y3 } = triangleData;
    
        // Начинаем новый путь
        context.beginPath();
        // Перемещаемся к первой точке треугольника
        context.moveTo(x1, y1);
        // Рисуем линии к остальным точкам треугольника
        context.lineTo(x2, y2);
        context.lineTo(x3, y3);
        // Закрываем путь, чтобы треугольник был замкнут
        context.closePath();
        // Заливаем треугольник цветом
        context.fillStyle = 'red';
        context.fill();


        
        return (<canvas ref={this.state.canvas} width={400} height={400} />);
    }
}


export default Garden;