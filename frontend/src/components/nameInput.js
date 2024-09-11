import React, { useState } from 'react';

export default function NameInput({value, setValue}) {

    const handleChange = (event) => setValue(event.target.value);

    return <input type="text" value={value} onChange={handleChange}></input>

}