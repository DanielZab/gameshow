import React, { useState } from 'react';

export default function RoleRadioInput({value, setValue}) {

    const handleChange = (event) => setValue(event.target.value);

    return <>
        <label>
            <input
                type="radio"
                value="host"
                checked={value === 'host'}
                onChange={handleChange}
            />
            Host
        </label>

        <label>
            <input
                type="radio"
                value="player"
                checked={value === 'player'}
                onChange={handleChange}
            />
            Player
        </label>

        <div>
            <p>You have selected: {value}</p>
        </div>

    </>

}