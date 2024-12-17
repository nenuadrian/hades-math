package com.nenuadrian;

import javafx.animation.PauseTransition;
import javafx.application.Application;
import javafx.geometry.Pos;
import javafx.scene.Scene;
import javafx.scene.control.*;
import javafx.scene.layout.*;
import javafx.stage.Stage;
import javafx.util.Duration;

public class Main extends Application {

    private static final int MATRIX_SIZE = 2; // Fixed size for simplicity
    private TextField[][] matrixAFields = new TextField[MATRIX_SIZE][MATRIX_SIZE];
    private TextField[][] matrixBFields = new TextField[MATRIX_SIZE][MATRIX_SIZE];
    private Label[][] matrixCLabels = new Label[MATRIX_SIZE][MATRIX_SIZE];
    private GridPane matrixAGrid, matrixBGrid, matrixCGrid;
    private Pane animationPane;

    @Override
    public void start(Stage stage) {
        // Root layout
        VBox root = new VBox(20);
        root.setAlignment(Pos.CENTER);

        // Input and result grids
        HBox gridsBox = new HBox(20);
        gridsBox.setAlignment(Pos.CENTER);
        animationPane = new Pane(); // Layer for animations

        matrixAGrid = createEditableMatrixGrid(matrixAFields, "Matrix A");
        matrixBGrid = createEditableMatrixGrid(matrixBFields, "Matrix B");
        matrixCGrid = createResultMatrixGrid(matrixCLabels, "Result Matrix C");

        gridsBox.getChildren().addAll(matrixAGrid, matrixBGrid, matrixCGrid);

        // Start button
        Button startButton = new Button("Start Multiplication");
        startButton.setOnAction(e -> startMultiplication());

        root.getChildren().addAll(gridsBox, startButton, animationPane);

        // Scene and stage
        Scene scene = new Scene(root, 1000, 600);
        stage.setScene(scene);
        stage.setTitle("Matrix Multiplication Visualizer");
        stage.show();
    }

    // Create an editable matrix grid
    private GridPane createEditableMatrixGrid(TextField[][] fields, String title) {
        GridPane grid = new GridPane();
        grid.setAlignment(Pos.CENTER);
        grid.setHgap(10);
        grid.setVgap(10);

        Label titleLabel = new Label(title);
        titleLabel.setStyle("-fx-font-size: 18;");
        grid.add(titleLabel, 0, 0, MATRIX_SIZE, 1);

        for (int i = 0; i < MATRIX_SIZE; i++) {
            for (int j = 0; j < MATRIX_SIZE; j++) {
                TextField field = new TextField("0");
                field.setMaxWidth(50);
                fields[i][j] = field;
                grid.add(field, j, i + 1);
            }
        }
        return grid;
    }

    // Create a result matrix grid
    private GridPane createResultMatrixGrid(Label[][] labels, String title) {
        GridPane grid = new GridPane();
        grid.setAlignment(Pos.CENTER);
        grid.setHgap(10);
        grid.setVgap(10);

        Label titleLabel = new Label(title);
        titleLabel.setStyle("-fx-font-size: 18;");
        grid.add(titleLabel, 0, 0, MATRIX_SIZE, 1);

        for (int i = 0; i < MATRIX_SIZE; i++) {
            for (int j = 0; j < MATRIX_SIZE; j++) {
                Label label = new Label("0");
                label.setStyle("-fx-border-color: black; -fx-padding: 10; -fx-font-size: 16;");
                labels[i][j] = label;
                grid.add(label, j, i + 1);
            }
        }
        return grid;
    }

    // Parse matrix input from TextFields
    private int[][] parseMatrix(TextField[][] fields) {
        int[][] matrix = new int[MATRIX_SIZE][MATRIX_SIZE];
        for (int i = 0; i < MATRIX_SIZE; i++) {
            for (int j = 0; j < MATRIX_SIZE; j++) {
                try {
                    matrix[i][j] = Integer.parseInt(fields[i][j].getText());
                } catch (NumberFormatException e) {
                    matrix[i][j] = 0; // Default to 0 if input is invalid
                }
            }
        }
        return matrix;
    }

    // Start the multiplication with animation
    private void startMultiplication() {
        int[][] A = parseMatrix(matrixAFields);
        int[][] B = parseMatrix(matrixBFields);
        int[][] C = new int[MATRIX_SIZE][MATRIX_SIZE];

        animationPane.getChildren().clear();

        // Step-by-step animation
        int delay = 0;
        for (int i = 0; i < MATRIX_SIZE; i++) {
            for (int j = 0; j < MATRIX_SIZE; j++) {
                int row = i, col = j;
                PauseTransition pause = new PauseTransition(Duration.millis(delay * 500));
                pause.setOnFinished(e -> animateCellMultiplication(row, col, A, B, C));
                pause.play();
                delay++;
            }
        }
    }

    // Animate the multiplication for a single cell
    private void animateCellMultiplication(int row, int col, int[][] A, int[][] B, int[][] C) {
        int result = 0;
        for (int k = 0; k < MATRIX_SIZE; k++) {
            result += A[row][k] * B[k][col];
        }
        C[row][col] = result;
        matrixCLabels[row][col].setText(String.valueOf(result));
    }

    public static void main(String[] args) {
        launch(args);
    }
}
