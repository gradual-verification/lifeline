import './Button.scss';

type Props = {onClick?: (event: React.MouseEvent<HTMLButtonElement, MouseEvent>) => void, disabled?: boolean, children: any, className?:string};

export const Button  = ({onClick, disabled, children, className}:Props) => {

    return (
        <button 
            disabled={disabled}
            className={'button ' + className}
            onClick={onClick}>
                {children}
        </button>
    );
}