import numpy as np

def dot_product(matrix, vector):
    """
    takes 2 arguments: matrix (a list of lists representing the matrix) and vector (a list representing the vector).
    Checks if the dimensions are compatible, and if so, iterates through each row and computes the dot product
    """
    if len(matrix[0]) != len(vector): # check if dimensions are compatible for a dot product
        return -1  # return -1 if dimensions aren't compatible
    
    result = []
    for row in matrix: 
        dot_product_value = sum(row[i] * vector[i] for i in range(len(vector))) # perform dot product for each row in the matrix w/the vector
        result.append(dot_product_value)
    
    return result

# a = [[1, 2], [2, 4]]
# b = [1, 2]
# print(dot_product(a, b))  # [5, 10]

def covariance_matrix_numpy(vectors):
    data = np.array(vectors) # convert list of vectors into a NumPy array
    
    cov_matrix = np.cov(data, rowvar=False, bias=True) # calculate covariance matrix using NumPy fn
    
    return cov_matrix.tolist()

# vectors = [[1, 2, 3], [4, 5, 6]]
# print(covariance_matrix_numpy(vectors))

def mean(vector): # calculate mean of a vector
    return sum(vector) / len(vector)

def covariance_matrix_python(vectors):
    """
    Uses nested loops iterate through each pair of features (i, j) to compute the covariance
    Returns a covariance matrix as a list of lists

    Assumes that all vectors have the same length
    """
    num_features = len(vectors)
    num_obs = len(vectors[0])  # this assumes all vectors have the same length
    
    means = [mean(feature) for feature in vectors] # calculate means for each feature
    
    cov_matrix = [[0] * num_features for _ in range(num_features)] # init covariance matrix w/0s
    
    for i in range(num_features): # calculate covariance
        for j in range(num_features):
            cov = sum((vectors[i][k] - means[i]) * (vectors[j][k] - means[j]) for k in range(num_obs)) / (num_obs - 1) # compute covariance btw feature i and feature j
            cov_matrix[i][j] = cov
    
    return cov_matrix

# vectors = [[1, 2, 3], [4, 5, 6]]
# print(covariance_matrix_python(vectors))

def jacobi_method(A, b, n_iter=10):
    """
    An iterative technique that iterates and rounds each intermediate solution to solve a system of linear equations of the form 𝐴𝑥=𝑏.
    Returns the rounded solution vector x as the approx solution to the system of linear equations.

    Takes three arguments: A: The coefficient matrix, b: The constant vector, n_iter (optional): number of iterations to perform (default is 10).
    Adjust n_iter based on the desired accuracy and convergence
    """

    n = len(A)
    x = [0] * n  # initialize x with zeros
    
    for _ in range(n_iter):
        x_new = x.copy()  # create a copy of x to store the new values
        for i in range(n):
            sum_ax = sum(A[i][j] * x[j] for j in range(n) if j != i)
            x_new[i] = (b[i] - sum_ax) / A[i][i]
        x = x_new
    
    x = [round(xi, 4) for xi in x] # round each element of x to 4 decimal places
    
    return x

# A = [[5, -2, 3], [-3, 9, 1], [2, -1, -7]]
# b = [-1, 2, 3]
# n_iter = 10
# print(jacobi_method(A, b, n_iter))

import math

def jacobi_svd_approximation(a, n_iter=10):
    """
    Computes rotation matrices 𝑈new 𝑉new based on the current state of matrix 𝐴 by using the Jacobi method to iteratively approximate 𝑈, Σ, and 𝑉
    Outputs the approximated 𝑈, Σ, and 𝑉 matrices
    """
    U = [[1, 0], [0, 1]] # init matrices U, Sigma, V
    V = [[1, 0], [0, 1]]
    
    # init diagonal elements of Sigma
    sigma = [0, 0]
    
    # Jacobi method to approximate SVD
    for _ in range(n_iter):
        theta = 0.5 * math.atan(2 * a[0][1] / (a[0][0] - a[1][1])) # 1: Compute U matrix
        c = math.cos(theta)
        s = math.sin(theta)
        U_new = [[c, -s], [s, c]]
        U = matrix_mult(U, U_new)
        
        a = matrix_mult(matrix_transpose(U_new), a) # 2: Update A with U^T * A
        a = matrix_mult(a, U_new)
        
        theta = 0.5 * math.atan(2 * a[0][1] / (a[0][0] - a[1][1])) # 3: Compute V matrix
        c = math.cos(theta)
        s = math.sin(theta)
        V_new = [[c, -s], [s, c]]
        V = matrix_mult(V_new, V)
        
        a = matrix_mult(a, matrix_transpose(V_new)) # 4: Update A with A * V
        a = matrix_mult(V_new, a)
    
    sigma[0] = a[0][0] # Sigma is the diagonal of A
    sigma[1] = a[1][1]
    
    return U, sigma, V

def matrix_mult(a, b):
    rows_a = len(a) # mat mult
    cols_a = len(a[0])
    cols_b = len(b[0])
    
    result = [[0] * cols_b for _ in range(rows_a)]
    
    for i in range(rows_a):
        for j in range(cols_b):
            for k in range(cols_a):
                result[i][j] += a[i][k] * b[k][j]
    
    return result

def matrix_transpose(matrix):
    return [[matrix[j][i] for j in range(len(matrix))] for i in range(len(matrix[0]))] # transpose matrix

# a = [[2, 1], [1, 2]]
# U, sigma, V = jacobi_svd_approximation(a)
# print("U:")
# for row in U:
#     print(row)
# print("Sigma:", sigma)
# print("V:")
# for row in V:
#     print(row)

def determinant_4x4(matrix):
    """
    Recursively calculates the determinant of a 4x4 matrix using the Laplace expansion method.
    Returns the determinant of the input matrix.
    """
    if len(matrix) != 4 or any(len(row) != 4 for row in matrix): # check if the matrix is 4x4
        raise ValueError("Input matrix must be a 4x4 matrix.")
    
    def determinant_2x2(mat): # base case: determinant of 2x2 matrix
        return mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0]
    
    def determinant_recursive(mat): # recurse to calculate determinant
        size = len(mat)
        
        if size == 2: # base case: 2x2 matrix
            return determinant_2x2(mat)
        
        det = 0
        for col in range(size): # iterate over the elements of the first row to calculate cofactors
            minor = [row[:col] + row[col+1:] for row in mat[1:]] # calc minor matrix without current row and column            
            det += mat[0][col] * ((-1) ** col) * determinant_recursive(minor) # calc determinant of minor matrix
        
        return det
    
    return determinant_recursive(matrix) 

# a = [[1, 2, 3, 4],
#      [5, 6, 7, 8],
#      [9, 10, 11, 12],
#      [13, 14, 15, 16]]

# print(determinant_4x4(a))  # 0

def transpose_matrix(matrix):
    """
    Transposes a matrix by swapping the rows and columns, populating it by iterating through and assigning the values by flipping the rows and columns
    Returns the transposed matrix
    Handles matrices with different numbers of rows and columns, as long as they are rectangular
    """
    rows = len(matrix) # num of rows and columns in the original matrix
    cols = len(matrix[0])
    
    transpose = [[0] * rows for _ in range(cols)] # init an empty matrix with swapped dimensions
    
    for i in range(rows): # fill the transpose matrix
        for j in range(cols):
            transpose[j][i] = matrix[i][j]
    
    return transpose

# a = [[1, 2, 3],
#      [4, 5, 6]]

# print(transpose_matrix(a))  # [[1, 4], [2, 5], [3, 6]]

def mean_matrix(matrix, mode='row'):
    """
    Calculates the mean of the rows or columns of a matrix, depending on the mode specified.
    Returns a list of means, either by row or by column.
    """
    if mode not in ['row', 'column']:
        raise ValueError("Mode must be either 'row' or 'column'.")
    
    num_rows = len(matrix)
    num_cols = len(matrix[0])
    
    means = []
    
    if mode == 'row':
        for row in matrix: # calculate mean by row
            row_mean = sum(row) / num_cols
            means.append(row_mean)
    
    elif mode == 'column':
        for j in range(num_cols): # calculate mean by column
            col_sum = sum(matrix[i][j] for i in range(num_rows))
            col_mean = col_sum / num_rows
            means.append(col_mean)
    
    return means

# matrix1 = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
# print(mean_matrix(matrix1, mode='column'))  # [4.0, 5.0, 6.0]
# print(mean_matrix(matrix1, mode='row'))     # [2.0, 5.0, 8.0]
