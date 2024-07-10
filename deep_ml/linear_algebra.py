import numpy as np

def dot_product(matrix, vector):
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

def mean(vector):
    return sum(vector) / len(vector)

def covariance_matrix_python(vectors):
    num_features = len(vectors)
    num_obs = len(vectors[0])  # this assumes all vectors have the same length
    
    means = [mean(feature) for feature in vectors] # calculate means for each feature
    
    cov_matrix = [[0] * num_features for _ in range(num_features)] # init covariance matrix w/0s
    
    for i in range(num_features): # calculate covariance
        for j in range(num_features):
            cov = sum((vectors[i][k] - means[i]) * (vectors[j][k] - means[j]) for k in range(num_obs)) / (num_obs - 1) # compute covariance btw feature i and feature j
            cov_matrix[i][j] = cov
    
    return cov_matrix

vectors = [[1, 2, 3], [4, 5, 6]]
print(covariance_matrix_python(vectors))
