import type { ReactNode } from "react";

interface StatusDotProps {
  color: "green" | "red" | "gray" | "amber";
  children?: ReactNode;
}

const COLORS: Record<StatusDotProps["color"], string> = {
  green: "#22c55e",
  red: "#ef4444",
  gray: "#9ca3af",
  amber: "#f59e0b",
};

export function StatusDot({ color, children }: StatusDotProps) {
  return (
    <span className="status-item">
      <span
        className="dot"
        style={{
          backgroundColor: COLORS[color],
          display: "inline-block",
          width: 10,
          height: 10,
          borderRadius: "50%",
          marginRight: 6,
        }}
        aria-hidden
      />
      {children}
    </span>
  );
}
