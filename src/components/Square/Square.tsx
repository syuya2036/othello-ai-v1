import "./Square.css";

interface SquareProps {
  value: string;
  canPut?: boolean;
  red?: boolean;
  onClick: () => void;
}

const Square: React.FC<SquareProps> = ({ value, onClick, canPut, red }) => {
  const name = canPut ? "square can-put" : "square";
  return (
    <button
      className={name}
      onClick={onClick}
      style={{ border: red ? "1px solid red" : "1px solid black" }}
    >
      {value}
    </button>
  );
};

export default Square;
