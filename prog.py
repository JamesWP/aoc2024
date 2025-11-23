import re
import os

def generate_dot_content(rules_string):
    """
    Parses logic gate rules and generates the content for a DOT graph file.

    The expected format for each rule is: <input_A> <OP> <input_B> -> <output_wire>
    Where OP is AND, OR, or XOR.
    """
    dot_lines = []
    gate_counter = 0
    all_wires = set()
    parsed_rules = []

    # Regex to capture: (Input A) (OP) (Input B) -> (Output Z)
    # Allows for flexible spacing
    rule_regex = re.compile(r'(\w+)\s+(AND|OR|XOR)\s+(\w+)\s+->\s+(\w+)', re.IGNORECASE)

    # --- 1. Parsing Rules and Identifying all Wires ---
    for line in rules_string.strip().split('\n'):
        line = line.strip()
        if not line:
            continue

        match = rule_regex.match(line)
        if match:
            # Groups: (Input A, Operation, Input B, Output Z)
            input_a, op, input_b, output_z = match.groups()
            op = op.upper() # Ensure consistent casing
            parsed_rules.append((input_a, op, input_b, output_z))

            # Track all unique wire names for node definitions
            all_wires.add(input_a)
            all_wires.add(input_b)
            all_wires.add(output_z)
        else:
            print(f"Warning: Skipped unparsable line: {line}")


    # --- 2. DOT Graph Definition Start and Styling ---
    dot_lines.append("digraph LogicCircuit {")
    dot_lines.append("    // Graph Settings for clean, left-to-right visualization")
    dot_lines.append("    rankdir=LR;")
    dot_lines.append('    graph [splines=polyline, bgcolor="#2e3440", fontname="Arial", fontcolor="#eceff4", compound=true];')
    dot_lines.append('    node [style="filled, rounded", fontname="Arial", color="#4c566a", penwidth=2];')
    dot_lines.append('    edge [fontname="Arial", color="#d8dee9", arrowhead=vee];')
    dot_lines.append("")

    # --- 3. Define Wire Nodes (Inputs and Outputs) ---
    dot_lines.append('    // Wire Nodes (Representing Inputs and Outputs)')
    if all_wires:
        wire_definition = '    ' + ' '.join([
            f'"{w}" [shape=oval, fillcolor="#88c0d0", fontcolor="#2e3440"];'
            for w in sorted(list(all_wires))
        ])
        dot_lines.append(wire_definition)
    dot_lines.append("")

    # --- 4. Generate Gate Nodes and Edges ---
    dot_lines.append('    // Gate Definitions and Connections')
    for input_a, op, input_b, output_z in parsed_rules:
        gate_counter += 1
        # Create a unique ID for the gate node
        gate_id = f"G{gate_counter}"

        # Assign color based on the gate type
        fill_color = ""
        if op == "AND":
            fill_color = "#a3be8c" # Green
        elif op == "OR":
            fill_color = "#b48ead" # Purple
        elif op == "XOR":
            fill_color = "#ebcb8b" # Yellow

        # Define the Gate Node (Box shape)
        dot_lines.append(f'    {gate_id} [label="{op}", shape=box, fillcolor="{fill_color}", fontcolor="#2e3440", style="filled, rounded"];')

        # Edges from Inputs to Gate
        # Using the unique wire name as the source node
        dot_lines.append(f'    "{input_a}" -> {gate_id} [label="A", color="#e5e9f0"];')
        dot_lines.append(f'    "{input_b}" -> {gate_id} [label="B", color="#e5e9f0"];')

        # Edge from Gate to Output
        dot_lines.append(f'    {gate_id} -> "{output_z}" [label="Z", color="#81a1c1", penwidth=1.5];')
        dot_lines.append("")


    # --- 5. DOT Graph Definition End ---
    dot_lines.append("}")

    return "\n".join(dot_lines)

def main():
    """
    Main function to run the DOT file generation, now reading rules from a user-specified file.
    """
    # Original rules commented out for reference, now the script expects a file path.
    # rules_input = """..."""

    # 1. Get the input file path from the user
    input_filename = input("Enter the filepath for the circuit rules (e.g., 'rules.txt'): ")

    # 2. Read the rules from the file
    rules_input = ""
    try:
        with open(input_filename, 'r') as f:
            rules_input = f.read()
            print(f"Successfully read rules from '{input_filename}'.")
    except FileNotFoundError:
        print(f"Error: The file '{input_filename}' was not found.")
        return
    except IOError as e:
        print(f"An error occurred while reading the file: {e}")
        return

    # 3. Generate and save the DOT file
    output_filename = "logic_circuit.dot"

    # Generate the DOT content
    dot_content = generate_dot_content(rules_input)

    # Save the DOT content to a file
    try:
        with open(output_filename, 'w') as f:
            f.write(dot_content)
        print(f"\nSuccessfully generated DOT file: '{output_filename}'")
        print("You can use a tool like Graphviz (e.g., 'dot -Tpng logic_circuit.dot -o circuit.png') to visualize it.")
    except IOError as e:
        print(f"Error saving output file: {e}")

if __name__ == "__main__":
    main()
