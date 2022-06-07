import React from 'react';
import './Navbar.scss';

type Props = {children: any, className?: string}
function Navbar({children, className}:Props) {
  return (
    <div className="Navbar">
        {children}
    </div>
  );
}

export default Navbar;
