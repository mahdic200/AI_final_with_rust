import numpy as np
import matplotlib.pyplot as plt
class queen:
    def __init__(self,n=8,popsize=2,pm=.01,maxiter=10):
        self.length=n
        self.maxiter=maxiter
        self.population=[]
        self.pm=pm
        self.popsize=popsize
        for i in range(self.popsize):
            self.population.append(np.random.permutation(list(range(1,self.length+1))))
    # parent selection inspected successfully
    def parentselection(self):
        # this is the fitness array associated with they'r
        # index with parents array
        fit=self.pop_fit()
        # i don't know why but in this line
        # we are calculating a ratio between each fitness and
        # summation of all fitnesses in fit array (discussed about fit array before)
        # so , the fit array now has ratio inside , for rollet , 
        # and each parent has a ratio fitness instead of just a fitness number
        fit=fit/np.sum(fit)
        # in this section we are making a new empty array
        # with zeros (probably for coming population)
        fitcum = np.zeros(self.popsize)
        # storing good indexes
        idx = np.zeros(self.popsize,dtype=int)
        # initiating first fitcum cell with fit array first index
        fitcum[0]=fit[0]
        # looping through fit array and summing each two
        # index together , for rollet , 
        # here attention that each parent must have more p
        for i in range(self.popsize-1):
            fitcum[i+1]=fitcum[i]+fit[i+1]
        # I figured out , implementing a random rollet !!!
        for i in range(self.popsize):
            num=np.random.rand()
            for j in range(len(fitcum)):
                if num<fitcum[j]:
                    idx[i]=j
                    break
        parents=[]
        for i in range(len(idx)):
            parents.append(self.population[idx[i]])
        return np.array(parents)
    def fitness(self,x):
        count=0
        for i in range(len(x)-1):
            for j in range(i+1,len(x)):
                if abs(i-j)==abs(x[i]-x[j]):
                    count+=1
        return 1/(count+0.000001)
    def swapmutation(self,x):
        if np.random.rand()<=self.pm:
            i=np.random.randint(len(x))
            j = np.random.randint(len(x))
            t=x[i]
            x[i]=x[j]
            x[j]=t
        return x
    def crossover(self,x,y):
       # print(x)
        #print(y)
        ind1 = np.random.randint(len(x))
        ind2 = np.random.randint(len(x))
        #print('ind1=',ind1,' ind2=',ind2)
        if ind2<ind1:
            t=ind1;ind1=ind2;ind2=t;
        child1 = np.zeros_like(x)
        child2 = np.zeros_like(x)
        for i in range(ind1,ind2+1):
            child1[i]=x[i]
        pointer1 = (ind2 + 1) % self.length
        pointer2 = (ind2 + 1) % self.length
        while 0 in child1:
            if y[pointer2] not in child1:
                child1[pointer1]=y[pointer2]
                pointer1 = (pointer1 + 1) % self.length
            pointer2 = (pointer2 + 1) % self.length

        for i in range(ind1, ind2 + 1):
            child2[i] = y[i]
        pointer1 = (ind2 + 1) % self.length
        pointer2 = (ind2 + 1) % self.length
        while 0 in child2:
            if x[pointer2] not in child2:
                child2[pointer1] = x[pointer2]
                pointer1 = (pointer1 + 1) % self.length
            pointer2 = (pointer2 + 1) % self.length
        #print('childs\n',child1,'\n',child2)
        return child1 , child2
    # returns an array including fitness quantity of each
    # parent , associated with parents array
    def pop_fit(self):
        fit=np.zeros(len(self.population))
        for i in range(len(self.population)):
            fit[i]=self.fitness(self.population[i])
        return fit
    def recombination(self,parents):
        offsprings=np.zeros_like(parents)
        for i in range(0,len(parents)-1,2):
            child1 , child2=self.crossover(parents[i],parents[i+1])
            offsprings[i,:]=child1
            offsprings[i+1,:]=child2
        return offsprings
    def mutation(self,offsprings):
        for i in range(len(offsprings)):
            offsprings[i,:]=self.swapmutation(offsprings[i,:])
        return offsprings



    def cycle(self):
        bestfit=np.zeros(self.maxiter)
        for i in range(self.maxiter):
            parents=self.parentselection()
            offsprings=self.recombination(parents)
            offsprings=self.mutation(offsprings)
            self.population=offsprings
            fits = self.pop_fit()
            bestfit[i]=np.max(fits)
            best=self.population[np.argmax(fits)]
            #print(self.population[np.argmax(fits)])
        plt.plot(bestfit)
        plt.show()
        print(best)

obj=queen(8,popsize=50,pm=.1,maxiter=1000)
#print(obj.population)
obj.cycle()






