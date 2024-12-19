import { useState } from 'react';
import axios from 'axios';

function App() {
    const [a, setA] = useState('');
    const [b, setB] = useState('');
    const [result, setResult] = useState(null);

    const handleSubmit = async (e) => {
        e.preventDefault();
        try {
            const response = await axios.post('http://127.0.0.1:8080/add', {
                a: parseInt(a),
                b: parseInt(b),
            });
            // console.log(response);
            setResult(response.data.data);
        } catch (error) {
            console.error('Error adding numbers:', error);
        }
    };

    return (
        <div>
            <h1>Add Two Numbers</h1>
            <form onSubmit={handleSubmit}>
                <input
                    type="number"
                    value={a}
                    onChange={(e) => setA(e.target.value)}
                    placeholder="Enter first number"
                    required
                />
                <input
                    type="number"
                    value={b}
                    onChange={(e) => setB(e.target.value)}
                    placeholder="Enter second number"
                    required
                />
                <button type="submit">Add</button>
            </form>
            {result !== null && <h2>Result: {result}</h2>}
        </div>
    );
}

export default App;